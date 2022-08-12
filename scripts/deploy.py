textFile = '''
#!/bin/bash
set -e
export ACCOUNT_ID=nearlend-official.testnet
export CONTRACT_ID=nft.$ACCOUNT_ID
export TREASURY_ID=$ACCOUNT_ID
export ROY1=$ACCOUNT_ID
echo $CONTRACT_ID
echo $ACCOUNT_ID

# near delete $CONTRACT_ID $ACCOUNT_ID
near create-account $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 40
near deploy $CONTRACT_ID --accountId $ACCOUNT_ID --wasmFile ../out/main.wasm

near call $CONTRACT_ID  new_default_meta '{"owner_id":"'$ACCOUNT_ID'", "treasury_id":"'$TREASURY_ID'"}' ---accountId $ACCOUNT_ID
'''

for x in range(1, 51):
    print(x)
    if x < 10:
        textFile = textFile + '''near call $CONTRACT_ID nft_create_series '{"token_series_id":"''' + str(
            x) + '''", "creator_id":"'$ACCOUNT_ID'","token_metadata":{"title": "Lang Biang - 00''' + str(
            x) + '''","media":"https://bafybeibkhcuq6s3drxtvyegtooc6wosudvbpzvhf5gtfy5xnwemp7mku6u.ipfs.nftstorage.link/Lang%20Biang%20%2300''' + str(
            x) + '''.png", "reference":"bafkreiceiix3c3q6eahprmo2aovs46sdkrfcrdz52c2ouojkul7uzk5zsa", "copies": 1},"price":"5000000000000000000000000", "royalty":{"'$ROY1'": 500}}' ---accountId $ACCOUNT_ID  --depositYocto 8540000000000000000000
        '''
    else:
        textFile = textFile + '''near call $CONTRACT_ID nft_create_series '{"token_series_id":"''' + str(
            x) + '''", "creator_id":"'$ACCOUNT_ID'","token_metadata":{"title": "Lang Biang - 0''' + str(
            x) + '''","media":"https://bafybeibkhcuq6s3drxtvyegtooc6wosudvbpzvhf5gtfy5xnwemp7mku6u.ipfs.nftstorage.link/Lang%20Biang%20%230''' + str(
            x) + '''.png", "reference":"bafkreiceiix3c3q6eahprmo2aovs46sdkrfcrdz52c2ouojkul7uzk5zsa", "copies": 1},"price":"5000000000000000000000000", "royalty":{"'$ROY1'": 500}}' ---accountId $ACCOUNT_ID  --depositYocto 8540000000000000000000
        '''
    textFile = textFile + '''
    near call $CONTRACT_ID nft_mint '{"token_series_id":"''' + str(
        x) + '''","receiver_id": "'$ACCOUNT_ID'"}' --accountId $ACCOUNT_ID --depositYocto 11280000000000000000000
    '''

# print(textFile)
f = open("deploy.sh", "w")
f.write(textFile)
f.close()