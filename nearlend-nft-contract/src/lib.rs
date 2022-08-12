use near_contract_standards::non_fungible_token::core::{
    NonFungibleTokenCore, NonFungibleTokenResolver,
};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::env::is_valid_account_id;
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, serde_json::json, AccountId, Balance,
    BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue, Timestamp,
};
use std::collections::HashMap;

pub mod event;

pub use event::NearEvent;

/// between token_series_id and edition number e.g. 42:2 where 42 is series and 2 is edition
pub const TOKEN_DELIMETER: char = ':';
/// TokenMetadata.title returned for individual token e.g. "Title — 2/10" where 10 is max copies
pub const TITLE_DELIMETER: &str = " #";
/// e.g. "Title — 2/10" where 10 is max copies
pub const EDITION_DELIMETER: &str = "/";

const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;
const GAS_FOR_NFT_TRANSFER_CALL: Gas = 30_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;
const GAS_FOR_NFT_APPROVE: Gas = 10_000_000_000_000;
const GAS_FOR_MINT: Gas = 90_000_000_000_000;
const NO_DEPOSIT: Balance = 0;
const MAX_PRICE: Balance = 1_000_000_000 * 10u128.pow(24);

pub type TokenSeriesId = String;
pub type TimestampSec = u32;
pub type ContractAndTokenId = String;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    /// Returns `true` if the token should be returned back to the sender.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> Promise;
}

#[ext_contract(ext_approval_receiver)]
pub trait NonFungibleTokenReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool;
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenSeries {
    metadata: TokenMetadata,
    creator_id: AccountId,
    tokens: UnorderedSet<TokenId>,
    price: Option<Balance>,
    is_mintable: bool,
    royalty: HashMap<AccountId, u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenSeriesJson {
    token_series_id: TokenSeriesId,
    metadata: TokenMetadata,
    creator_id: AccountId,
    royalty: HashMap<AccountId, u32>,
    transaction_fee: Option<U128>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransactionFee {
    pub next_fee: Option<u16>,
    pub start_time: Option<TimestampSec>,
    pub current_fee: u16,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MarketDataTransactionFee {
    pub transaction_fee: UnorderedMap<TokenSeriesId, u128>,
}

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ContractV1 {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    // CUSTOM
    token_series_by_id: UnorderedMap<TokenSeriesId, TokenSeries>,
    treasury_id: AccountId,
    transaction_fee: TransactionFee,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    // CUSTOM
    token_series_by_id: UnorderedMap<TokenSeriesId, TokenSeries>,
    treasury_id: AccountId,
    transaction_fee: TransactionFee,
    market_data_transaction_fee: MarketDataTransactionFee,
}

const DATA_IMAGE_SVG_NEARLEND_ICON: &str = "data:image/png;base64,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";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    // CUSTOM
    TokenSeriesById,
    TokensBySeriesInner { token_series: String },
    TokensPerOwner { account_hash: Vec<u8> },
    MarketDataTransactionFee,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId, treasury_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            treasury_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Lang Biang Collectibles".to_string(),
                symbol: "NLLANG".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEARLEND_ICON.to_string()),
                base_uri: Some("https://ipfs.fleek.co/ipfs".to_string()),
                reference: None,
                reference_hash: None,
            },
            0,
        )
    }

    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        treasury_id: ValidAccountId,
        metadata: NFTContractMetadata,
        current_fee: u16,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            token_series_by_id: UnorderedMap::new(StorageKey::TokenSeriesById),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            treasury_id: treasury_id.to_string(),
            transaction_fee: TransactionFee {
                next_fee: None,
                start_time: None,
                current_fee,
            },
            market_data_transaction_fee: MarketDataTransactionFee {
                transaction_fee: UnorderedMap::new(StorageKey::MarketDataTransactionFee),
            },
        }
    }

    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let prev: ContractV1 = env::state_read().expect("ERR_NOT_INITIALIZED");
        assert_eq!(
            env::predecessor_account_id(),
            prev.tokens.owner_id,
            "Nearlend Dao Only owner"
        );

        let this = Contract {
            tokens: prev.tokens,
            metadata: prev.metadata,
            token_series_by_id: prev.token_series_by_id,
            treasury_id: prev.treasury_id,
            transaction_fee: prev.transaction_fee,
            market_data_transaction_fee: MarketDataTransactionFee {
                transaction_fee: UnorderedMap::new(StorageKey::MarketDataTransactionFee),
            },
        };

        this
    }

    #[payable]
    pub fn set_transaction_fee(&mut self, next_fee: u16, start_time: Option<TimestampSec>) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Nearlend Dao Owner only"
        );

        assert!(
            next_fee < 10_000,
            "Nearlend Dao transaction fee is more than 10_000"
        );

        if start_time.is_none() {
            self.transaction_fee.current_fee = next_fee;
            self.transaction_fee.next_fee = None;
            self.transaction_fee.start_time = None;
            return;
        } else {
            let start_time: TimestampSec = start_time.unwrap();
            assert!(
                start_time > to_sec(env::block_timestamp()),
                "start_time is less than current block_timestamp"
            );
            self.transaction_fee.next_fee = Some(next_fee);
            self.transaction_fee.start_time = Some(start_time);
        }
    }

    pub fn calculate_market_data_transaction_fee(
        &mut self,
        token_series_id: &TokenSeriesId,
    ) -> u128 {
        if let Some(transaction_fee) = self
            .market_data_transaction_fee
            .transaction_fee
            .get(&token_series_id)
        {
            return transaction_fee;
        }

        // fallback to default transaction fee
        self.calculate_current_transaction_fee()
    }

    pub fn calculate_current_transaction_fee(&mut self) -> u128 {
        let transaction_fee: &TransactionFee = &self.transaction_fee;
        if transaction_fee.next_fee.is_some() {
            if to_sec(env::block_timestamp()) >= transaction_fee.start_time.unwrap() {
                self.transaction_fee.current_fee = transaction_fee.next_fee.unwrap();
                self.transaction_fee.next_fee = None;
                self.transaction_fee.start_time = None;
            }
        }
        self.transaction_fee.current_fee as u128
    }

    pub fn get_transaction_fee(&self) -> &TransactionFee {
        &self.transaction_fee
    }

    pub fn get_market_data_transaction_fee(&self, token_series_id: &TokenId) -> u128 {
        if let Some(transaction_fee) = self
            .market_data_transaction_fee
            .transaction_fee
            .get(&token_series_id)
        {
            return transaction_fee;
        }
        // fallback to default transaction fee
        self.transaction_fee.current_fee as u128
    }

    // Treasury
    #[payable]
    pub fn set_treasury(&mut self, treasury_id: ValidAccountId) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Nearlend Dao Owner only"
        );
        self.treasury_id = treasury_id.to_string();
    }

    // CUSTOM

    #[payable]
    pub fn nft_create_series(
        &mut self,
        creator_id: Option<ValidAccountId>,
        token_metadata: TokenMetadata,
        price: Option<U128>,
        royalty: Option<HashMap<AccountId, u32>>,
    ) -> TokenSeriesJson {
        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();

        if creator_id.is_some() {
            assert_eq!(
                creator_id.unwrap().to_string(),
                caller_id,
                "Nearlend Dao Caller is not creator_id"
            );
        }

        let token_series_id = format!("{}", (self.token_series_by_id.len() + 1));

        assert!(
            self.token_series_by_id.get(&token_series_id).is_none(),
            "Nearlend Dao duplicate token_series_id"
        );

        let title = token_metadata.title.clone();
        assert!(
            title.is_some(),
            "Nearlend Dao token_metadata.title is required"
        );

        let mut total_perpetual = 0;
        let mut total_accounts = 0;
        let royalty_res: HashMap<AccountId, u32> = if let Some(royalty) = royalty {
            for (k, v) in royalty.iter() {
                if !is_valid_account_id(k.as_bytes()) {
                    env::panic("Not valid account_id for royalty".as_bytes());
                };
                total_perpetual += *v;
                total_accounts += 1;
            }
            royalty
        } else {
            HashMap::new()
        };

        assert!(
            total_accounts <= 10,
            "Nearlend Dao royalty exceeds 10 accounts"
        );

        assert!(
            total_perpetual <= 9000,
            "Nearlend Dao Exceeds maximum royalty -> 9000",
        );

        let price_res: Option<u128> = if price.is_some() {
            assert!(
                price.unwrap().0 < MAX_PRICE,
                "Nearlend Dao price higher than {}",
                MAX_PRICE
            );
            Some(price.unwrap().0)
        } else {
            None
        };

        self.token_series_by_id.insert(
            &token_series_id,
            &TokenSeries {
                metadata: token_metadata.clone(),
                creator_id: caller_id.to_string(),
                tokens: UnorderedSet::new(
                    StorageKey::TokensBySeriesInner {
                        token_series: token_series_id.clone(),
                    }
                    .try_to_vec()
                    .unwrap(),
                ),
                price: price_res,
                is_mintable: true,
                royalty: royalty_res.clone(),
            },
        );

        // set market data transaction fee
        let current_transaction_fee = self.calculate_current_transaction_fee();
        self.market_data_transaction_fee
            .transaction_fee
            .insert(&token_series_id, &current_transaction_fee);

        env::log(
            json!({
                "type": "nft_create_series",
                "params": {
                    "token_series_id": token_series_id,
                    "token_metadata": token_metadata,
                    "creator_id": caller_id,
                    "price": price,
                    "royalty": royalty_res,
                    "transaction_fee": &current_transaction_fee.to_string()
                }
            })
            .to_string()
            .as_bytes(),
        );

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        TokenSeriesJson {
            token_series_id,
            metadata: token_metadata,
            creator_id: caller_id.into(),
            royalty: royalty_res,
            transaction_fee: Some(current_transaction_fee.into()),
        }
    }

    #[payable]
    pub fn nft_buy(
        &mut self,
        token_series_id: TokenSeriesId,
        receiver_id: ValidAccountId,
    ) -> TokenId {
        let initial_storage_usage = env::storage_usage();

        let token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Nearlend Dao Token series not exist");
        let price: u128 = token_series.price.expect("Nearlend Dao not for sale");
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= price,
            "Nearlend Dao attached deposit is less than price : {}",
            price
        );
        let token_id: TokenId =
            self._nft_mint_series(token_series_id.clone(), receiver_id.to_string());

        let for_treasury = price as u128
            * self.calculate_market_data_transaction_fee(&token_series_id)
            / 10_000u128;
        let price_deducted = price - for_treasury;
        Promise::new(token_series.creator_id).transfer(price_deducted);

        if for_treasury != 0 {
            Promise::new(self.treasury_id.clone()).transfer(for_treasury);
        }

        refund_deposit(env::storage_usage() - initial_storage_usage, price);

        NearEvent::log_nft_mint(
            receiver_id.to_string(),
            vec![token_id.clone()],
            Some(json!({"price": price.to_string()}).to_string()),
        );

        token_id
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_series_id: TokenSeriesId,
        receiver_id: ValidAccountId,
    ) -> TokenId {
        let initial_storage_usage = env::storage_usage();

        let token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Nearlend Dao Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Nearlend Dao not creator"
        );
        let token_id: TokenId = self._nft_mint_series(token_series_id, receiver_id.to_string());

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        NearEvent::log_nft_mint(receiver_id.to_string(), vec![token_id.clone()], None);

        token_id
    }

    #[payable]
    pub fn nft_mint_and_approve(
        &mut self,
        token_series_id: TokenSeriesId,
        account_id: ValidAccountId,
        msg: Option<String>,
    ) -> Option<Promise> {
        let initial_storage_usage = env::storage_usage();

        let token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Nearlend Dao Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Nearlend Dao not creator"
        );
        let token_id: TokenId =
            self._nft_mint_series(token_series_id, token_series.creator_id.clone());

        // Need to copy the nft_approve code here to solve the gas problem
        // get contract-level LookupMap of token_id to approvals HashMap
        let approvals_by_id = self.tokens.approvals_by_id.as_mut().unwrap();

        // update HashMap of approvals for this token
        let approved_account_ids = &mut approvals_by_id
            .get(&token_id)
            .unwrap_or_else(|| HashMap::new());
        let account_id: AccountId = account_id.into();
        let approval_id: u64 = self
            .tokens
            .next_approval_id_by_id
            .as_ref()
            .unwrap()
            .get(&token_id)
            .unwrap_or_else(|| 1u64);
        approved_account_ids.insert(account_id.clone(), approval_id);

        // save updated approvals HashMap to contract's LookupMap
        approvals_by_id.insert(&token_id, &approved_account_ids);

        // increment next_approval_id for this token
        self.tokens
            .next_approval_id_by_id
            .as_mut()
            .unwrap()
            .insert(&token_id, &(approval_id + 1));

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        NearEvent::log_nft_mint(
            token_series.creator_id.clone(),
            vec![token_id.clone()],
            None,
        );

        if let Some(msg) = msg {
            Some(ext_approval_receiver::nft_on_approve(
                token_id,
                token_series.creator_id,
                approval_id,
                msg,
                &account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE - GAS_FOR_MINT,
            ))
        } else {
            None
        }
    }

    fn _nft_mint_series(
        &mut self,
        token_series_id: TokenSeriesId,
        receiver_id: AccountId,
    ) -> TokenId {
        let mut token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Nearlend Dao Token series not exist");
        assert!(
            token_series.is_mintable,
            "Nearlend Dao Token series is not mintable"
        );

        let num_tokens = token_series.tokens.len();
        let max_copies = token_series.metadata.copies.unwrap_or(u64::MAX);
        assert!(num_tokens < max_copies, "Series supply maxed");

        if (num_tokens + 1) >= max_copies {
            token_series.is_mintable = false;
        }

        let token_id = format!("{}{}{}", &token_series_id, TOKEN_DELIMETER, num_tokens + 1);
        token_series.tokens.insert(&token_id);
        self.token_series_by_id
            .insert(&token_series_id, &token_series);

        // you can add custom metadata to each token here
        let metadata = Some(TokenMetadata {
            title: None,       // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
            description: None, // free-form description
            media: None, // URL to associated media, preferably to decentralized, content-addressed storage
            media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
            copies: None, // number of copies of this set of metadata in existence when token was minted.
            issued_at: Some(env::block_timestamp().to_string()), // ISO 8601 datetime when token was issued or minted
            expires_at: None,     // ISO 8601 datetime when token expires
            starts_at: None,      // ISO 8601 datetime when token starts being valid
            updated_at: None,     // ISO 8601 datetime when token was last updated
            extra: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
            reference: None, // URL to an off-chain JSON file with more info.
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        });

        //let token = self.tokens.mint(token_id, receiver_id, metadata);
        // From : https://github.com/near/near-sdk-rs/blob/master/near-contract-standards/src/non_fungible_token/core/core_impl.rs#L359
        // This allows lazy minting

        let owner_id: AccountId = receiver_id;
        self.tokens.owner_by_id.insert(&token_id, &owner_id);

        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata.as_ref().unwrap()));

        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TokensPerOwner {
                    account_hash: env::sha256(&owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }

        token_id
    }

    #[payable]
    pub fn nft_set_series_non_mintable(&mut self, token_series_id: TokenSeriesId) {
        assert_one_yocto();

        let mut token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Nearlend Dao Creator only"
        );

        assert_eq!(
            token_series.is_mintable, true,
            "Nearlend Dao already non-mintable"
        );

        assert_eq!(
            token_series.metadata.copies, None,
            "Nearlend Dao decrease supply if copies not null"
        );

        token_series.is_mintable = false;
        self.token_series_by_id
            .insert(&token_series_id, &token_series);
        env::log(
            json!({
                "type": "nft_set_series_non_mintable",
                "params": {
                    "token_series_id": token_series_id,
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    #[payable]
    pub fn nft_decrease_series_copies(
        &mut self,
        token_series_id: TokenSeriesId,
        decrease_copies: U64,
    ) -> U64 {
        assert_one_yocto();

        let mut token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Nearlend Dao Creator only"
        );

        let minted_copies = token_series.tokens.len();
        let copies = token_series.metadata.copies.unwrap();

        assert!(
            (copies - decrease_copies.0) >= minted_copies,
            "Nearlend Dao cannot decrease supply, already minted : {}",
            minted_copies
        );

        let is_non_mintable = if (copies - decrease_copies.0) == minted_copies {
            token_series.is_mintable = false;
            true
        } else {
            false
        };

        token_series.metadata.copies = Some(copies - decrease_copies.0);

        self.token_series_by_id
            .insert(&token_series_id, &token_series);
        env::log(
            json!({
                "type": "nft_decrease_series_copies",
                "params": {
                    "token_series_id": token_series_id,
                    "copies": U64::from(token_series.metadata.copies.unwrap()),
                    "is_non_mintable": is_non_mintable,
                }
            })
            .to_string()
            .as_bytes(),
        );
        U64::from(token_series.metadata.copies.unwrap())
    }

    #[payable]
    pub fn nft_set_series_price(
        &mut self,
        token_series_id: TokenSeriesId,
        price: Option<U128>,
    ) -> Option<U128> {
        assert_one_yocto();

        let mut token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Nearlend Dao Creator only"
        );

        assert_eq!(
            token_series.is_mintable, true,
            "Nearlend Dao token series is not mintable"
        );

        if price.is_none() {
            token_series.price = None;
        } else {
            assert!(
                price.unwrap().0 < MAX_PRICE,
                "Nearlend Dao price higher than {}",
                MAX_PRICE
            );
            token_series.price = Some(price.unwrap().0);
        }

        self.token_series_by_id
            .insert(&token_series_id, &token_series);

        // set market data transaction fee
        let current_transaction_fee = self.calculate_current_transaction_fee();
        self.market_data_transaction_fee
            .transaction_fee
            .insert(&token_series_id, &current_transaction_fee);

        env::log(
            json!({
                "type": "nft_set_series_price",
                "params": {
                    "token_series_id": token_series_id,
                    "price": price,
                    "transaction_fee": current_transaction_fee.to_string()
                }
            })
            .to_string()
            .as_bytes(),
        );
        return price;
    }

    #[payable]
    pub fn nft_burn(&mut self, token_id: TokenId) {
        assert_one_yocto();

        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        assert_eq!(owner_id, env::predecessor_account_id(), "Token owner only");

        if let Some(next_approval_id_by_id) = &mut self.tokens.next_approval_id_by_id {
            next_approval_id_by_id.remove(&token_id);
        }

        if let Some(approvals_by_id) = &mut self.tokens.approvals_by_id {
            approvals_by_id.remove(&token_id);
        }

        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap();
            token_ids.remove(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }

        if let Some(token_metadata_by_id) = &mut self.tokens.token_metadata_by_id {
            token_metadata_by_id.remove(&token_id);
        }

        self.tokens.owner_by_id.remove(&token_id);

        NearEvent::log_nft_burn(owner_id, vec![token_id], None, None);
    }

    // CUSTOM VIEWS

    pub fn nft_get_series_single(&self, token_series_id: TokenSeriesId) -> TokenSeriesJson {
        let token_series = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("Series does not exist");
        let current_transaction_fee = self.get_market_data_transaction_fee(&token_series_id);
        TokenSeriesJson {
            token_series_id,
            metadata: token_series.metadata,
            creator_id: token_series.creator_id,
            royalty: token_series.royalty,
            transaction_fee: Some(current_transaction_fee.into()),
        }
    }

    pub fn nft_get_series_format(self) -> (char, &'static str, &'static str) {
        (TOKEN_DELIMETER, TITLE_DELIMETER, EDITION_DELIMETER)
    }

    pub fn nft_get_series_price(self, token_series_id: TokenSeriesId) -> Option<U128> {
        let price = self.token_series_by_id.get(&token_series_id).unwrap().price;
        match price {
            Some(p) => return Some(U128::from(p)),
            None => return None,
        };
    }

    pub fn nft_get_series(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<TokenSeriesJson> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.token_series_by_id.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        self.token_series_by_id
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_series_id, token_series)| TokenSeriesJson {
                token_series_id,
                metadata: token_series.metadata,
                creator_id: token_series.creator_id,
                royalty: token_series.royalty,
                transaction_fee: None,
            })
            .collect()
    }

    pub fn nft_supply_for_series(&self, token_series_id: TokenSeriesId) -> U64 {
        self.token_series_by_id
            .get(&token_series_id)
            .expect("Token series not exist")
            .tokens
            .len()
            .into()
    }

    pub fn nft_tokens_by_series(
        &self,
        token_series_id: TokenSeriesId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        let tokens = self
            .token_series_by_id
            .get(&token_series_id)
            .unwrap()
            .tokens;
        assert!(
            (tokens.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        tokens
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|token_id| self.nft_token(token_id).unwrap())
            .collect()
    }

    pub fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        let owner_id = self.tokens.owner_by_id.get(&token_id)?;
        let approved_account_ids = self
            .tokens
            .approvals_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id).or_else(|| Some(HashMap::new())));

        // CUSTOM (switch metadata for the token_series metadata)
        let mut token_id_iter = token_id.split(TOKEN_DELIMETER);
        let token_series_id = token_id_iter.next().unwrap().parse().unwrap();
        let series_metadata = self
            .token_series_by_id
            .get(&token_series_id)
            .unwrap()
            .metadata;

        let mut token_metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .unwrap()
            .get(&token_id)
            .unwrap();

        token_metadata.title = Some(format!(
            "{}{}{}",
            series_metadata.title.unwrap(),
            TITLE_DELIMETER,
            token_id_iter.next().unwrap()
        ));

        token_metadata.reference = series_metadata.reference;
        token_metadata.media = series_metadata.media;
        token_metadata.copies = series_metadata.copies;
        token_metadata.extra = series_metadata.extra;

        Some(Token {
            token_id,
            owner_id,
            metadata: Some(token_metadata),
            approved_account_ids,
        })
    }

    // CUSTOM core standard repeated here because no macro below

    pub fn nft_transfer_unsafe(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        let sender_id = env::predecessor_account_id();
        let receiver_id_str = receiver_id.to_string();
        let (previous_owner_id, _) = self.tokens.internal_transfer(
            &sender_id,
            &receiver_id_str,
            &token_id,
            approval_id,
            memo.clone(),
        );

        let authorized_id: Option<AccountId> = if sender_id != previous_owner_id {
            Some(sender_id)
        } else {
            None
        };

        NearEvent::log_nft_transfer(
            previous_owner_id,
            receiver_id_str,
            vec![token_id],
            memo,
            authorized_id,
        );
    }

    #[payable]
    pub fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        let sender_id = env::predecessor_account_id();
        let previous_owner_id = self
            .tokens
            .owner_by_id
            .get(&token_id)
            .expect("Token not found");
        let receiver_id_str = receiver_id.to_string();
        self.tokens
            .nft_transfer(receiver_id, token_id.clone(), approval_id, memo.clone());

        let authorized_id: Option<AccountId> = if sender_id != previous_owner_id {
            Some(sender_id)
        } else {
            None
        };

        NearEvent::log_nft_transfer(
            previous_owner_id,
            receiver_id_str,
            vec![token_id],
            memo,
            authorized_id,
        );
    }

    #[payable]
    pub fn nft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let (previous_owner_id, old_approvals) = self.tokens.internal_transfer(
            &sender_id,
            receiver_id.as_ref(),
            &token_id,
            approval_id,
            memo.clone(),
        );

        let authorized_id: Option<AccountId> = if sender_id != previous_owner_id {
            Some(sender_id.clone())
        } else {
            None
        };

        NearEvent::log_nft_transfer(
            previous_owner_id.clone(),
            receiver_id.to_string(),
            vec![token_id.clone()],
            memo,
            authorized_id,
        );

        // Initiating receiver's call and the callback
        ext_non_fungible_token_receiver::nft_on_transfer(
            sender_id,
            previous_owner_id.clone(),
            token_id.clone(),
            msg,
            receiver_id.as_ref(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
        )
        .then(ext_self::nft_resolve_transfer(
            previous_owner_id,
            receiver_id.into(),
            token_id,
            old_approvals,
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
        .into()
    }

    // CUSTOM enumeration standard modified here because no macro below

    pub fn nft_total_supply(&self) -> U128 {
        (self.tokens.owner_by_id.len() as u128).into()
    }

    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        // Get starting index, whether or not it was explicitly given.
        // Defaults to 0 based on the spec:
        // https://nomicon.io/Standards/NonFungibleToken/Enumeration.html#interface
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.tokens.owner_by_id.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        self.tokens
            .owner_by_id
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_id, _)| self.nft_token(token_id).unwrap())
            .collect()
    }

    pub fn nft_supply_for_owner(self, account_id: ValidAccountId) -> U128 {
        let tokens_per_owner = self.tokens.tokens_per_owner.expect(
            "Could not find tokens_per_owner when calling a method on the enumeration standard.",
        );
        tokens_per_owner
            .get(account_id.as_ref())
            .map(|account_tokens| U128::from(account_tokens.len() as u128))
            .unwrap_or(U128(0))
    }

    pub fn nft_tokens_for_owner(
        &self,
        account_id: ValidAccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        let tokens_per_owner = self.tokens.tokens_per_owner.as_ref().expect(
            "Could not find tokens_per_owner when calling a method on the enumeration standard.",
        );
        let token_set = if let Some(token_set) = tokens_per_owner.get(account_id.as_ref()) {
            token_set
        } else {
            return vec![];
        };
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            token_set.len() as u128 > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        token_set
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|token_id| self.nft_token(token_id).unwrap())
            .collect()
    }

    pub fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout {
        let owner_id = self.tokens.owner_by_id.get(&token_id).expect("No token id");
        let mut token_id_iter = token_id.split(TOKEN_DELIMETER);
        let token_series_id = token_id_iter.next().unwrap().parse().unwrap();
        let royalty = self
            .token_series_by_id
            .get(&token_series_id)
            .expect("no type")
            .royalty;

        assert!(
            royalty.len() as u32 <= max_len_payout,
            "Market cannot payout to that many receivers"
        );

        let balance_u128: u128 = balance.into();

        let mut payout: Payout = Payout {
            payout: HashMap::new(),
        };
        let mut total_perpetual = 0;

        for (k, v) in royalty.iter() {
            if *k != owner_id {
                let key = k.clone();
                payout
                    .payout
                    .insert(key, royalty_to_payout(*v, balance_u128));
                total_perpetual += *v;
            }
        }
        payout.payout.insert(
            owner_id,
            royalty_to_payout(10000 - total_perpetual, balance_u128),
        );
        payout
    }

    #[payable]
    pub fn nft_transfer_payout(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        balance: Option<U128>,
        max_len_payout: Option<u32>,
    ) -> Option<Payout> {
        assert_one_yocto();

        let sender_id = env::predecessor_account_id();
        // Transfer
        let previous_token = self.nft_token(token_id.clone()).expect("no token");
        self.tokens
            .nft_transfer(receiver_id.clone(), token_id.clone(), approval_id, None);

        // Payout calculation
        let previous_owner_id = previous_token.owner_id;
        let mut total_perpetual = 0;
        let payout = if let Some(balance) = balance {
            let balance_u128: u128 = u128::from(balance);
            let mut payout: Payout = Payout {
                payout: HashMap::new(),
            };

            let mut token_id_iter = token_id.split(TOKEN_DELIMETER);
            let token_series_id = token_id_iter.next().unwrap().parse().unwrap();
            let royalty = self
                .token_series_by_id
                .get(&token_series_id)
                .expect("no type")
                .royalty;

            assert!(
                royalty.len() as u32 <= max_len_payout.unwrap(),
                "Market cannot payout to that many receivers"
            );
            for (k, v) in royalty.iter() {
                let key = k.clone();
                if key != previous_owner_id {
                    payout
                        .payout
                        .insert(key, royalty_to_payout(*v, balance_u128));
                    total_perpetual += *v;
                }
            }

            assert!(total_perpetual <= 10000, "Total payout overflow");

            payout.payout.insert(
                previous_owner_id.clone(),
                royalty_to_payout(10000 - total_perpetual, balance_u128),
            );
            Some(payout)
        } else {
            None
        };

        let authorized_id: Option<AccountId> = if sender_id != previous_owner_id {
            Some(sender_id)
        } else {
            None
        };

        NearEvent::log_nft_transfer(
            previous_owner_id,
            receiver_id.to_string(),
            vec![token_id],
            None,
            authorized_id,
        );

        payout
    }

    pub fn get_owner(&self) -> AccountId {
        self.tokens.owner_id.clone()
    }
}

fn royalty_to_payout(a: u32, b: Balance) -> U128 {
    U128(a as u128 * b / 10_000u128)
}

// near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
// near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool {
        let resp: bool = self.tokens.nft_resolve_transfer(
            previous_owner_id.clone(),
            receiver_id.clone(),
            token_id.clone(),
            approved_account_ids,
        );

        // if not successful, return nft back to original owner
        if !resp {
            NearEvent::log_nft_transfer(receiver_id, previous_owner_id, vec![token_id], None, None);
        }

        resp
    }
}

/// from https://github.com/near/near-sdk-rs/blob/e4abb739ff953b06d718037aa1b8ab768db17348/near-contract-standards/src/non_fungible_token/utils.rs#L29

fn refund_deposit(storage_used: u64, extra_spend: Balance) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit() - extra_spend;

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

fn to_sec(timestamp: Timestamp) -> TimestampSec {
    (timestamp / 10u64.pow(9)) as u32
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    const STORAGE_FOR_CREATE_SERIES: Balance = 8540000000000000000000;
    const STORAGE_FOR_MINT: Balance = 11280000000000000000000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new_default_meta(accounts(0), accounts(4));
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(
            accounts(1),
            accounts(4),
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Triple Triad".to_string(),
                symbol: "TRIAD".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEARLEND_ICON.to_string()),
                base_uri: Some("https://ipfs.fleek.co/ipfs/".to_string()),
                reference: None,
                reference_hash: None,
            },
            500,
        );
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_owner(), accounts(1).to_string());
        assert_eq!(
            contract.nft_metadata().base_uri.unwrap(),
            "https://ipfs.fleek.co/ipfs/".to_string()
        );
        assert_eq!(
            contract.nft_metadata().icon.unwrap(),
            DATA_IMAGE_SVG_NEARLEND_ICON.to_string()
        );
    }

    fn create_series(
        contract: &mut Contract,
        royalty: &HashMap<AccountId, u32>,
        price: Option<U128>,
        copies: Option<u64>,
    ) {
        contract.nft_create_series(
            None,
            TokenMetadata {
                title: Some("Tsundere land".to_string()),
                description: None,
                media: Some(
                    "bafybeidzcan4nzcz7sczs4yzyxly4galgygnbjewipj6haco4kffoqpkiy".to_string(),
                ),
                media_hash: None,
                copies: copies,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: Some(
                    "bafybeicg4ss7qh5odijfn2eogizuxkrdh3zlv4eftcmgnljwu7dm64uwji".to_string(),
                ),
                reference_hash: None,
            },
            price,
            Some(royalty.clone()),
        );
    }

    #[test]
    fn test_create_series() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);
        create_series(
            &mut contract,
            &royalty,
            Some(U128::from(1 * 10u128.pow(24))),
            None,
        );

        let nft_series_return = contract.nft_get_series_single("1".to_string());
        assert_eq!(nft_series_return.creator_id, accounts(1).to_string());

        assert_eq!(nft_series_return.token_series_id, "1",);

        assert_eq!(nft_series_return.royalty, royalty,);

        assert!(nft_series_return.metadata.copies.is_none());

        assert_eq!(
            nft_series_return.metadata.title.unwrap(),
            "Tsundere land".to_string()
        );

        assert_eq!(
            nft_series_return.metadata.reference.unwrap(),
            "bafybeicg4ss7qh5odijfn2eogizuxkrdh3zlv4eftcmgnljwu7dm64uwji".to_string()
        );
    }

    #[test]
    fn test_buy() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(
            &mut contract,
            &royalty,
            Some(U128::from(1 * 10u128.pow(24))),
            None,
        );

        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1 * 10u128.pow(24) + STORAGE_FOR_MINT)
            .build());

        let token_id = contract.nft_buy("1".to_string(), accounts(2));

        let token_from_nft_token = contract.nft_token(token_id);
        assert_eq!(
            token_from_nft_token.unwrap().owner_id,
            accounts(2).to_string()
        )
    }

    #[test]
    fn test_mint() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, None);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        let token_id = contract.nft_mint("1".to_string(), accounts(2));

        let token_from_nft_token = contract.nft_token(token_id);
        assert_eq!(
            token_from_nft_token.unwrap().owner_id,
            accounts(2).to_string()
        )
    }

    #[test]
    #[should_panic(expected = "Nearlend Dao Token series is not mintable")]
    fn test_invalid_mint_non_mintable() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, None);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());
        contract.nft_set_series_non_mintable("1".to_string());

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        contract.nft_mint("1".to_string(), accounts(2));
    }

    #[test]
    #[should_panic(expected = "Nearlend Dao Token series is not mintable")]
    fn test_invalid_mint_above_copies() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, Some(1));

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        contract.nft_mint("1".to_string(), accounts(2));
        contract.nft_mint("1".to_string(), accounts(2));
    }

    #[test]
    fn test_decrease_copies() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, Some(5));

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        contract.nft_mint("1".to_string(), accounts(2));
        contract.nft_mint("1".to_string(), accounts(2));

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());

        contract.nft_decrease_series_copies("1".to_string(), U64::from(3));
    }

    #[test]
    #[should_panic(expected = "Nearlend Dao cannot decrease supply, already minted : 2")]
    fn test_invalid_decrease_copies() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, Some(5));

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        contract.nft_mint("1".to_string(), accounts(2));
        contract.nft_mint("1".to_string(), accounts(2));

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());

        contract.nft_decrease_series_copies("1".to_string(), U64::from(4));
    }

    #[test]
    #[should_panic(expected = "Nearlend Dao not for sale")]
    fn test_invalid_buy_price_null() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(
            &mut contract,
            &royalty,
            Some(U128::from(1 * 10u128.pow(24))),
            None,
        );

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());

        contract.nft_set_series_price("1".to_string(), None);

        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1 * 10u128.pow(24) + STORAGE_FOR_MINT)
            .build());

        let token_id = contract.nft_buy("1".to_string(), accounts(2));

        let token_from_nft_token = contract.nft_token(token_id);
        assert_eq!(
            token_from_nft_token.unwrap().owner_id,
            accounts(2).to_string()
        )
    }

    #[test]
    #[should_panic(expected = "Nearlend Dao price higher than 1000000000000000000000000000000000")]
    fn test_invalid_price_shouldnt_be_higher_than_max_price() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(
            &mut contract,
            &royalty,
            Some(U128::from(1_000_000_000 * 10u128.pow(24))),
            None,
        );

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());
    }

    #[test]
    fn test_nft_burn() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, None);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        let token_id = contract.nft_mint("1".to_string(), accounts(2));

        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1)
            .build());

        contract.nft_burn(token_id.clone());
        let token = contract.nft_token(token_id);
        assert!(token.is_none());
    }

    #[test]
    fn test_nft_transfer() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, None);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        let token_id = contract.nft_mint("1".to_string(), accounts(2));

        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1)
            .build());

        contract.nft_transfer(accounts(3), token_id.clone(), None, None);

        let token = contract.nft_token(token_id).unwrap();
        assert_eq!(token.owner_id, accounts(3).to_string())
    }

    #[test]
    fn test_nft_transfer_unsafe() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, None);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        let token_id = contract.nft_mint("1".to_string(), accounts(2));

        testing_env!(context.predecessor_account_id(accounts(2)).build());

        contract.nft_transfer_unsafe(accounts(3), token_id.clone(), None, None);

        let token = contract.nft_token(token_id).unwrap();
        assert_eq!(token.owner_id, accounts(3).to_string())
    }

    #[test]
    fn test_nft_transfer_payout() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        create_series(&mut contract, &royalty, None, None);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_MINT)
            .build());

        let token_id = contract.nft_mint("1".to_string(), accounts(2));

        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1)
            .build());

        let payout = contract.nft_transfer_payout(
            accounts(3),
            token_id.clone(),
            Some(0),
            Some(U128::from(1 * 10u128.pow(24))),
            Some(10),
        );

        let mut payout_calc: HashMap<AccountId, U128> = HashMap::new();
        payout_calc.insert(
            accounts(1).to_string(),
            U128::from((1000 * (1 * 10u128.pow(24))) / 10_000),
        );
        payout_calc.insert(
            accounts(2).to_string(),
            U128::from((9000 * (1 * 10u128.pow(24))) / 10_000),
        );

        assert_eq!(payout.unwrap().payout, payout_calc);

        let token = contract.nft_token(token_id).unwrap();
        assert_eq!(token.owner_id, accounts(3).to_string())
    }

    #[test]
    fn test_change_transaction_fee_immediately() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        contract.set_transaction_fee(100, None);

        assert_eq!(contract.get_transaction_fee().current_fee, 100);
    }

    #[test]
    fn test_change_transaction_fee_with_time() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        assert_eq!(contract.get_transaction_fee().current_fee, 500);
        assert_eq!(contract.get_transaction_fee().next_fee, None);
        assert_eq!(contract.get_transaction_fee().start_time, None);

        let next_fee: u16 = 100;
        let start_time: Timestamp = 1618109122863866400;
        let start_time_sec: TimestampSec = to_sec(start_time);
        contract.set_transaction_fee(next_fee, Some(start_time_sec));

        assert_eq!(contract.get_transaction_fee().current_fee, 500);
        assert_eq!(contract.get_transaction_fee().next_fee, Some(next_fee));
        assert_eq!(
            contract.get_transaction_fee().start_time,
            Some(start_time_sec)
        );

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .block_timestamp(start_time + 1)
            .build());

        contract.calculate_current_transaction_fee();
        assert_eq!(contract.get_transaction_fee().current_fee, next_fee);
        assert_eq!(contract.get_transaction_fee().next_fee, None);
        assert_eq!(contract.get_transaction_fee().start_time, None);
    }

    #[test]
    fn test_transaction_fee_locked() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        assert_eq!(contract.get_transaction_fee().current_fee, 500);
        assert_eq!(contract.get_transaction_fee().next_fee, None);
        assert_eq!(contract.get_transaction_fee().start_time, None);

        let next_fee: u16 = 100;
        let start_time: Timestamp = 1618109122863866400;
        let start_time_sec: TimestampSec = to_sec(start_time);
        contract.set_transaction_fee(next_fee, Some(start_time_sec));

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1).to_string(), 1000);

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build());

        create_series(
            &mut contract,
            &royalty,
            Some(U128::from(1 * 10u128.pow(24))),
            None,
        );

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        contract.nft_set_series_price("1".to_string(), None);

        assert_eq!(contract.get_transaction_fee().current_fee, 500);
        assert_eq!(contract.get_transaction_fee().next_fee, Some(next_fee));
        assert_eq!(
            contract.get_transaction_fee().start_time,
            Some(start_time_sec)
        );

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .block_timestamp(start_time + 1)
            .attached_deposit(1)
            .build());

        contract.calculate_current_transaction_fee();
        assert_eq!(contract.get_transaction_fee().current_fee, next_fee);
        assert_eq!(contract.get_transaction_fee().next_fee, None);
        assert_eq!(contract.get_transaction_fee().start_time, None);

        let series = contract.nft_get_series_single("1".to_string());
        let series_transaction_fee: u128 = series.transaction_fee.unwrap().into();
        assert_eq!(series_transaction_fee, 500);
    }
}
