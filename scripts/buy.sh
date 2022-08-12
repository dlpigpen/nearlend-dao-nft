
#!/bin/bash
set -e
export ACCOUNT_ID=nearlend-langbiang.testnet
export CONTRACT_ID=lang-biang6.$ACCOUNT_ID
export TREASURY_ID=$ACCOUNT_ID
export ROY1=$ACCOUNT_ID
echo $CONTRACT_ID
echo $ACCOUNT_ID

#near call $CONTRACT_ID nft_create_series '{"token_series_id":"3", "creator_id":"'$ACCOUNT_ID'","token_metadata":{"title": "Lang Biang - 002","media":"https://bafybeibkhcuq6s3drxtvyegtooc6wosudvbpzvhf5gtfy5xnwemp7mku6u.ipfs.nftstorage.link/Lang%20Biang%20%23002.png", "reference":"bafkreihrtxi4x6o4nkt24dgv3tuayllmdcxp7hzqjryb7gxp45edogte54", "copies": 1},"price":"5000000000000000000000000", "royalty":{"'$ROY1'": 500}}' ---accountId $ACCOUNT_ID  --depositYocto 8540000000000000000000
#near call $CONTRACT_ID nft_buy '{"token_series_id":"3","receiver_id":"mitsori9.testnet"}' --deposit 6 --accountId mitsori9.testnet
#export ACCOUNT_ID2=mitsori9.testnet
#near call marketplace.nearlend-official.testnet storage_deposit '{"accountId":"'$ACCOUNT_ID2'"}' --accountId $ACCOUNT_ID2 --deposit 5
#near call $CONTRACT_ID nft_approve '{"token_id":"3:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\",\"price\":\"1000000000000000000000000\",\"ft_token_id\":\"near\"}"}'  --accountId $ACCOUNT_ID2 --deposit 5


near view marketplace.nearlend-official.testnet get_market_data '{"nft_contract_id":"'$CONTRACT_ID'","token_id":"1:1"}'
#near view marketplace.nearlend-official.testnet get_market_data '{"nft_contract_id":"'$CONTRACT_ID'","token_id":"2:1"}'
#near view marketplace.nearlend-official.testnet get_market_data '{"nft_contract_id":"lang-biang4.nearlend-nft.testnet","token_id":"1:1"}'

#near call $CONTRACT_ID nft_approve '{"token_id":"2:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\",\"price\":\"1000000000000000000000000\",\"ft_token_id\":\"near\"}"}'  --accountId $ACCOUNT_ID --deposit 5

#near call --accountId mitsori9.testnet marketplace.nearlend-official.testnet buy '{"nft_contract_id":"'$CONTRACT_ID'","token_id":"1:1"}' --depositYocto 3000000000000000000000000 --gas 300000000000000
