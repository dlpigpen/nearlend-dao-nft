textFile = '''
#!/bin/bash
set -e
export ACCOUNT_ID=nearlend-nft2.testnet
export CONTRACT_ID=lang-biang5.$ACCOUNT_ID
export TREASURY_ID=$ACCOUNT_ID
export ROY1=$ACCOUNT_ID
echo $CONTRACT_ID
echo $ACCOUNT_ID
'''

for x in range(1, 3):
     textFile = textFile + '''near call $CONTRACT_ID nft_burn '{"token_id":"''' + str(x) + ''':1"}' ---accountId $ACCOUNT_ID --depositYocto 1
     '''
textFile =  textFile + '''
near delete $CONTRACT_ID $ACCOUNT_ID
'''
# print(textFile)
f = open("burn.sh", "w")
f.write(textFile)
f.close()