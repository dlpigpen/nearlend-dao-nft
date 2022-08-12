
#!/bin/bash
set -e
export ACCOUNT_ID=nearlend-nft2.testnet
export CONTRACT_ID=lang-biang5.$ACCOUNT_ID
export TREASURY_ID=$ACCOUNT_ID
export ROY1=$ACCOUNT_ID
echo $CONTRACT_ID
echo $ACCOUNT_ID
near call $CONTRACT_ID nft_burn '{"token_id":"1:1"}' ---accountId $ACCOUNT_ID --depositYocto 1
     
near delete $CONTRACT_ID $ACCOUNT_ID
