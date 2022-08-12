textFile = '''
#!/bin/bash
set -e
export ACCOUNT_ID=nearlend-official.testnet
export CONTRACT_ID=nft.$ACCOUNT_ID
export TREASURY_ID=$ACCOUNT_ID
export ROY1=$ACCOUNT_ID
echo $CONTRACT_ID
echo $ACCOUNT_ID

near call marketplace.nearlend-official.testnet storage_deposit '{"accountId":"'$ACCOUNT_ID'"}' --accountId $ACCOUNT_ID --deposit 5
'''

for x in range(1, 51):
    print(x)
    textFile = textFile + '''
    near call $CONTRACT_ID nft_approve '{"token_id":"'''+ str(x) + ''':1","account_id":"marketplace.nearlend-official.testnet","msg":"{\\"market_type\\":\\"sale\\", \\"price\\":\\"5000000000000000000000000\\",\\"ft_token_id\\":\\"near\\"}"}' --accountId $ACCOUNT_ID --depositYocto 1320000000000000000000
     '''

# print(textFile)
f = open("deploy_to_paras.sh", "w")
f.write(textFile)
f.close()