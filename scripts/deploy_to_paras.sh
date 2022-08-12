
#!/bin/bash
set -e
export ACCOUNT_ID=nearlend-official.testnet
export CONTRACT_ID=nft.$ACCOUNT_ID
export TREASURY_ID=$ACCOUNT_ID
export ROY1=$ACCOUNT_ID
echo $CONTRACT_ID
echo $ACCOUNT_ID

near call marketplace.nearlend-official.testnet storage_deposit '{"accountId":"'$ACCOUNT_ID'"}' --accountId $ACCOUNT_ID --deposit 5

    near call $CONTRACT_ID nft_approve '{"token_id":"1:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"2:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"3:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"4:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"5:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"6:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"7:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"8:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"9:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"10:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"11:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"12:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"13:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"14:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"15:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"16:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"17:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"18:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"19:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"20:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"21:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"22:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"23:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"24:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"25:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"26:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"27:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"28:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"29:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"30:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"31:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"32:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"33:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"34:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"35:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"36:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"37:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"38:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"39:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"40:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"41:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"42:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"43:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"44:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"45:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"46:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"47:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"48:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"49:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     
    near call $CONTRACT_ID nft_approve '{"token_id":"50:1","account_id":"marketplace.nearlend-official.testnet","msg":"{\"market_type\":\"sale\", \"price\":\"5000000000000000000000000\",\"ft_token_id\":\"near\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     