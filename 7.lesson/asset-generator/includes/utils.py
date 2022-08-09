from optparse import OptionParser
def Args(numberNFTs, testRarities, randomizeOutput):
    parser = OptionParser()
    
    parser.add_option('-p', '--public-nfts', dest = 'public_nfts', help = 'Number of candy machine NFTs for public mint. [default=0]', metavar = 'INT', type = 'int')
    parser.add_option('-w', '--whitelist-nfts', dest = 'whitelist_nfts', help = 'Number of candy machine NFTs for whitelisted mint. [default=0]', metavar = 'INT', type = 'int')
    parser.add_option('-g', '--giveaway-nfts', dest = 'giveaway_nfts', help = 'Number of candy machine NFTs for giveways. [default=0]', metavar = 'INT', type = 'int')
    parser.add_option('-t', '--test-rarities', dest = 'test_rarities', help = 'Calculate the rarities without creating NFTs. [default=0]', metavar = 'BOOL (0 or 1)', type = 'int')
    parser.add_option('-r', '--randomize-output', dest = 'randomize_output', help = 'Randomize the output of the nfts (f.e. the 0.json can be the NFT #123) [default = 0]', metavar = 'BOOL (0 or 1)', type = 'int')
    
    opts, args = parser.parse_args()

    if opts.public_nfts:
        numberNFTs[0] = max(0,int(opts.public_nfts))
    if opts.whitelist_nfts:
        numberNFTs[1] = max(0,int(opts.whitelist_nfts))
    if opts.giveaway_nfts:
        numberNFTs[2] = max(0,int(opts.giveaway_nfts))
    if opts.test_rarities:
        testRarities = bool(opts.test_rarities)
    if opts.randomize_output:
        randomizeOutput = bool(opts.randomize_output)

    if testRarities:
        print('TESTING RARITIES ACTIVE... RARITIES WILL BE CALCULATED USING A SAMPLE OF NFTs.')
        print('THIS VERSION WONT CREATE ANY METADATA PAIR.')
        print('USE IT AS A DEBUG ONLY OPTION TO SEE HOW UR TICKET_VALUES FROM UR INPUR ASSETS WORKS.')
        print('TO CALCULATE A REAL DISTRIBUTION OF UR ITEMS RARITY DONT USE THIS OPTION.')
        print()
    
    if randomizeOutput:
        print('RANDOMIZE OUTPUT ACTIVE... NFTs LOCATION WILL BE SHUFFLED BEFORE THE METADATA+PNG IS GENERATED.')
        print('THE MEANING OF THIS IS: YOUR 0.json THAT SHOULD BE NFT #0, CAN BE THE NFT #1234 AND SO ON.')
        print()

    return numberNFTs, testRarities, randomizeOutput
