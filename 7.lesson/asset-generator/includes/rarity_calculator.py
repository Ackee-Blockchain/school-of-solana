import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import os
import json

class RarityCalculator:
    colors = ''
    percentages = []
    rarities = []
    def __init__(self, nfts,colors, rarities, percentages):
        self.colors = colors
        self.percentages = percentages
        self.rarities = rarities
        self.percentagesList, self.rarityList, self.stockList = self.GetPercentagesList(nfts)
        self.colors = self.PlotPercentages(self.percentagesList, self.rarityList, nfts)
        self.CreateRaritiesJsonReadable(nfts, self.rarityList, self.percentagesList, self.stockList)
        self.CreateNFTRanking(nfts, self.percentagesList, self.stockList)

    def GetPercentagesList(self, nfts):
        print('Obtaining items percentages of ur NFTs...', end = ' ', flush = True)
        numberNFTs = sum(nfts.nftsQuantity)
        percentagesList = []
        rarityList = []
        stockList = []
        for i in range (len(nfts.attributes)):
            percentagesList.append(dict())
            rarityList.append(dict())
            stockList.append(dict())
            for item in nfts.items[i]:
                percentagesList[i][item] = 0
                rarityList[i][item] = 'Not created'
                stockList[i][item] = 0
        for nfts in nfts.nfts:
            for nft in nfts:
                for i in range (len(nft.attributes)):
                    percentagesList[i][nft.attributes[i][1]] += round(((1.0)/numberNFTs),4)
                    stockList[i][nft.attributes[i][1]] += 1
                    rarityList[i][nft.attributes[i][1]] = self.CheckRarities(percentagesList[i][nft.attributes[i][1]])
        print('Done.')
        return percentagesList, rarityList, stockList
    
    def CheckRarities(self, percentage):
        for i in range(len(self.percentages)):
            if (percentage * 100.0) < self.percentages[i]:
                return self.rarities[i]
        return self.rarities[-1]

    def PlotPercentages(self, percentagesList, rarityList, nfts):
        self.colors['Not created'] = '#000000'

        for i in range (len(nfts.attributes)):
            print('Plotting items rarities for',nfts.attributes[i]+'...', end = ' ', flush = True)
            fig, ax = plt.subplots(figsize=(10, 7), tight_layout=True)
            X = percentagesList[i].keys() 
            Y = [i * 100.0 for i in percentagesList[i].values()]
            color = [self.colors[rarityList[i][item]] for item in X]
            ax.bar(X,Y, color=color)
            ax.tick_params(axis='x', labelrotation= 45)
            ax.set_title(nfts.attributes[i])
        
            plt.savefig(os.path.dirname(__file__) + '/../output/rarity/plots/' + nfts.attributes[i]+'.png')
            print('Done.')
        print('Check output/rarity/plots folder to see ur attributes plot.')
        return self.colors

    def CreateRaritiesJsonReadable(self, nfts, rarityList, percentagesList, stockList):
        raritiesJson = []
        for i in range (len(nfts.attributes)):
            layerName = nfts.attributes[i]
            raritiesJson.append(dict())
            raritiesJson[i]['attribute'] = layerName
            raritiesJson[i]['items'] = []
            for j in range (len(nfts.items[i])):
                raritiesJson[i]['items'].append({
                    'name': nfts.items[i][j],
                    'stock': stockList[i][nfts.items[i][j]],
                    'total': nfts.nftsCreatedCounter,
                    'percentage': round(percentagesList[i][nfts.items[i][j]] * 100.0,4),
                    'rarity': rarityList[i][nfts.items[i][j]],
                    'color': self.colors[rarityList[i][nfts.items[i][j]]],
                    'path': 'assets/'+nfts.orderedLayersPath[i]+'/'+nfts.itemsPath[i][j]
                })
            raritiesJson[i]['items'] = sorted(raritiesJson[i]['items'], key=lambda d: d['percentage'])
        with open(os.path.dirname(__file__) + '/../output/rarity/raritiesToReactMap.json', 'w') as jsonFile:
            json.dump(raritiesJson, jsonFile, indent = 4)
        print('Check output/rarity/raritiesToReactMap.json to see the rarities of our items in a json file.')
    
    def CreateNFTRanking(self, nfts, percentageList, stockList):
        nftRarityJson = []
        for _nfts in nfts.nfts:
            for nft in _nfts:
                score = []
                score_dict = {}
                for i in range (len(nft.attributes)):
                    attribute = nft.attributes[i][0]
                    item = nft.attributes[i][1]
                    _score = round(1/(percentageList[i][item]), 4)
                    score.append(_score)
                    score_dict[attribute] = {
                            "item": item,
                            "total_stock": stockList[i][item],
                            "percentage": round(percentageList[i][item] * 100.0,4),
                            "score": _score
                        }
                nftRarityJson.append({
                    "name": nft.name,
                    "rank": 0,
                    "score": sum(score),
                    "score_distribution": score_dict,
                    "image_url": "",
                    "folder": nft.folder_path
                })
        nftRarityJson = sorted(nftRarityJson, key=lambda d: d['score'], reverse = True)

        for i in range(len(nftRarityJson)):
            nftRarityJson[i]["rank"] = i+1 

        with open(os.path.dirname(__file__) + '/../output/rarity/nftsRanking.json', 'w') as jsonFile:
            json.dump(nftRarityJson, jsonFile, indent = 4)
        print("Check output/rarity/nftsRanking.json to see the ranking of ur NFTs!.")