# NEARLEND DAO - NFT Series Implementation

## Instructions

`yarn && yarn test:deploy`

#### Pre-reqs

Rust, cargo, near-cli, etc...
Everything should work if you have NEAR development env for Rust contracts set up.

[Tests](tests/simulation_tests.rs)
[Contract](nearlend-nft-contract/src/lib.rs)

## Example Call

### Deploy

```
env NEAR_ENV=local near --keyPath ~/.near/localnet/validator_key.json deploy --accountId mitsori9.testnet
```

### NFT init

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori9.testnet mitsori9.testnet new_default_meta '{"owner_id":"mitsori9.testnet", "treasury_id":"treasury.test.near"}'
```

### NFT create series

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori9.testnet mitsori9.testnet nft_create_series '{"token_series_id":"1", "creator_id":"mitsori10.testnet","token_metadata":{"title":"Naruto Shippuden ch.2: Menolong sasuke","media":"bafybeidzcan4nzcz7sczs4yzyxly4galgygnbjewipj6haco4kffoqpkiy", "reference":"bafybeicg4ss7qh5odijfn2eogizuxkrdh3zlv4eftcmgnljwu7dm64uwji", "copies": 100},"price":"1000000000000000000000000"}' --depositYocto 8540000000000000000000
```

### NFT create series with royalty

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori9.testnet mitsori9.testnet nft_create_series '{"token_series_id":"1","creator_id":"mitsori10.testnet","token_metadata":{"title":"Naruto Shippuden ch.2: Menolong sasuke","media":"bafybeidzcan4nzcz7sczs4yzyxly4galgygnbjewipj6haco4kffoqpkiy", "reference":"bafybeicg4ss7qh5odijfn2eogizuxkrdh3zlv4eftcmgnljwu7dm64uwji", "copies": 100},"price":"1000000000000000000000000", "royalty":{"mitsori10.testnet": 1000}}' --depositYocto 8540000000000000000000
```

### NFT transfer with payout

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori9.testnet mitsori9.testnet nft_transfer_payout '{"token_id":"10:1","receiver_id":"comic1.test.near","approval_id":"0","balance":"1000000000000000000000000", "max_len_payout": 10}' --depositYocto 1
```

### NFT buy

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori9.testnet mitsori9.testnet nft_buy '{"token_series_id":"1","receiver_id":"mitsori9.testnet"}' --depositYocto 1011280000000000000000000
```

### NFT mint series (Creator only)

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori10.testnet mitsori9.testnet nft_mint '{"token_series_id":"1","receiver_id":"mitsori9.testnet"}' --depositYocto 11280000000000000000000
```

### NFT transfer

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori9.testnet mitsori9.testnet nft_transfer '{"token_id":"1:1","receiver_id":"comic1.test.near"}' --depositYocto 1
```

### NFT set series non mintable (Creator only)

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori10.testnet mitsori9.testnet nft_set_series_non_mintable '{"token_series_id":"1"}' --depositYocto 1
```

### NFT set series price (Creator only)

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori10.testnet mitsori9.testnet nft_set_series_price '{"token_series_id":"1", "price": "2000000000000000000000000"}' --depositYocto 1
```

### NFT set series not for sale (Creator only)

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori10.testnet mitsori9.testnet nft_set_series_price '{"token_series_id":"1"}' --depositYocto 1
```

### NFT burn

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori9.testnet mitsori9.testnet nft_burn '{"token_id":"1:1"}' --depositYocto 1
```

### NFT approve

```
env NEAR_ENV=local near call --keyPath ~/.near/localnet/validator_key.json --accountId mitsori10.testnet mitsori9.testnet nft_approve '{"token_id":"1:10","account_id":"marketplace.test.near","msg":"{\"price\":\"3000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --depositYocto 1320000000000000000000
```
