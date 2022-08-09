from includes.nft_creator import NftCreator
from includes.utils import Args
from includes.rarity_calculator import RarityCalculator

folderPaths = ['public_mint_assets','whitelist_mint_assets','giveaway_assets']
numberNFTs, testRarities, randomizedOutput = Args([0 for i in folderPaths], False, False)

colors = {
    'Legendary':'#ff8000',
    'Epic':'#a335ee',
    'Rare': '#0070dd',
    'Uncommon': '#6bca06',
    'Common': '#a0a0a0'
}

rarities = list(colors.keys())
percentages = [3.0, 6.5, 10.0, 17.0] 

nfts = NftCreator(numberNFTs, folderPaths, testRarities, randomizedOutput)

print()
print('-------------------------------------------------------------------------')
print()

rarities = RarityCalculator(nfts, colors, rarities, percentages)
