

textFile = ''''''
for x in range(1, 51):
    print(x)
    textFile = textFile + '''near call langbiang.nearlend-nft.testnet nft_transfer '{ "receiver_id": "mitsori11.testnet", "token_id": "'''+ str(x) + '''" }' --accountId $ACCOUNT_ID --depositYocto 1
    '''


# print(textFile)
f = open("scripts/transfer_nft.sh", "w")
f.write(textFile)
f.close()