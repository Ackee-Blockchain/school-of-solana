from .nft import Nft
from natsort import natsorted
import os
import json
import numpy as np
import shutil
import copy
class NftCreator:
    nftsCreatedCounter = 0
    nftsUniques = []
    nfts = []
    testRarities = False
    totalMetadata = []
    totalDNA = []
    def __init__(self, numberNFTs, folder_paths, testRarities, randomizeOutput):
        self.testRarities = testRarities
        self.CreateOutputFile()
        self.attributes, self.orderedLayersPath = self.GetAttributesList()
        self.items, self.itemsPath, self.itemsTombola, self.maxPossibilities = self.GetItemsList(self.orderedLayersPath,self.attributes)
        self.nftsQuantity = self.SetNftTotalQuantity(numberNFTs, self.maxPossibilities)
        self.jsonTemplate = self.GetJsonTemplate()
        for i in range(len(self.nftsQuantity)):
            if self.nftsQuantity[i] <= 0:
                self.nfts.append([])
                continue
            if not self.testRarities:
                os.mkdir(os.path.dirname(__file__) + '/../output/nfts/'+folder_paths[i])
            self.nfts.append(self.CreateNfts(self.nftsQuantity[i], self.jsonTemplate, self.attributes, self.orderedLayersPath, self.items, self.itemsPath, self.itemsTombola, folder_paths[i]))
        
        if randomizeOutput:
            self.nfts = self.ShuffleNfts(self.nftsQuantity)

        for i in range(len(self.nftsQuantity)):
            if self.nftsQuantity[i] <= 0:
                self.totalMetadata.append([])
                self.totalDNA.append([])
                continue
            if not self.testRarities:
                self.fMet, self.fDNA = self.CreateImageAndMetadata(i, self.nftsQuantity[i], folder_paths[i])
                self.totalMetadata.append(self.fMet)
                self.totalDNA.append(self.fDNA)
                self.CreateExtraJsonFiles(i, folder_paths[i])
        if not self.testRarities:
            self.CreateTotalNFTInfo()
            print('Check output/nfts folder to see ur', self.nftsCreatedCounter,'creations.')
        else:
            print('Rarities will be calculated using a sample of',  self.nftsCreatedCounter, 'nfts.')

    #Clean the output folder for obtain a new output
    def CreateOutputFile(self):
        if not self.testRarities:
            print('Cleaning output folder...', end = ' ', flush = True)
            
            shutil.rmtree(os.path.dirname(__file__) + '/../output')
            os.mkdir(os.path.dirname(__file__) + '/../output')
            os.mkdir(os.path.dirname(__file__) + '/../output/nfts')
        else:
            print('Cleaning output/rarity folder...', end = ' ', flush = True)
            shutil.rmtree(os.path.dirname(__file__) + '/../output/rarity')
        os.mkdir(os.path.dirname(__file__) + '/../output/rarity')
        os.mkdir(os.path.dirname(__file__) + '/../output/rarity/plots')
        print('Done.')

    #Returns the json template to use on our nfts
    def GetJsonTemplate(self):
        with open(os.path.dirname(__file__) + '/../input/template.json') as template:
            return json.load(template)

    #Creates the attributes list for our nfts and select the layer order that our nfts will use to create each nft
    def GetAttributesList(self):
        print('Obtainig attributes list and order of layers...', end = ' ', flush = True)
        orderedLayersPath = natsorted(os.listdir(os.path.dirname(__file__) + '/../input/assets'))
        if len(orderedLayersPath) <= 1:
            print('ERROR. You need at least 2 differents attributes.')
            exit()
        for file in orderedLayersPath:
            if(file[0] == '.'):
                orderedLayersPath.remove(file)
        attributes = []
        for file in orderedLayersPath:
            file = file.replace('_',' ').split('-')
            attributes.append(file[1].title())
        print("Done.")
        return attributes, orderedLayersPath


    def GetItemsList(self, attributesPath,attributes):
        items = []
        itemsPath = []
        itemsTombola = []
        maxPossibilities = 1
        for i in range(len(attributesPath)):
            item, itemPath, itemTombola = self.GetItemsPerAttribute(attributesPath[i], attributes[i])
            items.append(item)
            itemsPath.append(itemPath)
            itemsTombola.append(itemTombola)
            maxPossibilities = np.ulonglong(maxPossibilities * len(item))
        print('Calculating max possibilittes...',end = ' ', flush = True) 
        if maxPossibilities <= 1:
            print('ERROR. You just have', maxPossibilities, 'possibilites, create more assets')
            exit()
        print('You can create a max of', maxPossibilities, 'NFTs.')
        return items, itemsPath, itemsTombola, maxPossibilities


    def GetItemsPerAttribute(self, attributePath, attribute):
        print('Obtaining', attribute,'items and creating the tombola...', end = ' ', flush = True)
        files = natsorted(os.listdir(os.path.dirname(__file__) + '/../input/assets/' + attributePath))
        itemTombola = [0]
        item = []
        itemPath = files
        for file in files:
            if(file[0] == '.'):
                continue
            file = file.split('-')
            item.append(file[1].replace('.png', '').replace('_', ' ').title())
            itemTombola.append(itemTombola[-1] + int(file[0]))
        print('Done.')
        return item, itemPath, itemTombola

    def SetNftTotalQuantity(self, numberNFTs, maxPossibilities):
        nftTotalQuantity = 0
        percentagesOfUse = []
        nftTotalQuantity = sum(numberNFTs)
        for numberNFT in numberNFTs:
            percentagesOfUse.append(numberNFT/nftTotalQuantity)

        if nftTotalQuantity > maxPossibilities:
            nftTotalQuantity = int(0.7*maxPossibilities)
            for i in range(len(percentagesOfUse)):
                if percentagesOfUse[i] >= 0.0001:
                    numberNFTs[i] = max(1,round(percentagesOfUse[i]*nftTotalQuantity))
                else:
                    numberNFTs[i] = 0
            print('WARNING. Trying to create more NFTs than possibilites (' + str(maxPossibilities) + ' max possiblities), upload more accesories.', numberNFTs, '=', sum(numberNFTs), 'NFTs will be created instead.')
        elif nftTotalQuantity > int(maxPossibilities*0.8):
            print('WARNING. This may take several time. Trying to create', numberNFTs, '=', sum(numberNFTs), 'NFTs out of a maximum of', maxPossibilities,'.')
        else:
            print(numberNFTs, '=', sum(numberNFTs),' NFTs will be created.')
        return numberNFTs
        

    def CreateNfts(self, nftTotalQuantity, jsonTemplate, attributes, orderedLayersPath, items, itemsPath, itemsTombola, folder_path):
        print('Calculating NFTs unique configuration for', folder_path,'...', end = ' ', flush = True)
        nftsThisRun = []
        nftsCounterThisRun = 0
        hasDifferentBackground = False
        for nft in range (nftTotalQuantity):
            while True:
                hasDifferentBackground  = False
                nftDNA = []
                for i in range(len(attributes)):
                    randomUniformSelector = np.random.randint(0,itemsTombola[i][-1])
                    l = 0
                    r = len(itemsTombola[i]) - 1
                    while l <= r:
                        mid = l + int(((r - l) / 2))
                        if itemsTombola[i][mid] <= randomUniformSelector and itemsTombola[i][mid+1] > randomUniformSelector:
                            nftDNA.append(mid)
                            break
                        elif itemsTombola[i][mid] <= randomUniformSelector and itemsTombola[i][mid+1] <= randomUniformSelector:
                            l = mid + 1
                        elif itemsTombola[i][mid] > randomUniformSelector and itemsTombola[i][mid+1] > randomUniformSelector:
                            r = mid - 1
                #Same image with just a different background!
                for nftt in self.nftsUniques:
                    if nftDNA[1:] == nftt[1:]:
                        hasDifferentBackground  = True
                        break
                if not hasDifferentBackground:
                    break
            self.nftsUniques.append(nftDNA)
            nftsThisRun.append(nftDNA)
        print('Done.')
        nftsCreated = []
        for dna in nftsThisRun:
            nftAttributes = []
            rawAttributes = []
            nftPaths = []
            attributes_count = 0
            filteredAttributes = []
            for i in range(len(attributes)):
                if items[i][dna[i]] != "Notrait":
                    attributes_count += 1
                    filteredAttributes.append({attributes[i]: items[i][dna[i]]})
                    nftAttributes.append({"trait_type": attributes[i], "value":items[i][dna[i]]})
                rawAttributes.append([attributes[i],items[i][dna[i]]])
                nftPaths.append(itemsPath[i][dna[i]])
            nftMetadata = dict(jsonTemplate)
            nftName = nftMetadata['name']
            nftMetadata['attributes'] = nftAttributes
            nftMetadata['name'] = nftName + ' #' + str(self.nftsCreatedCounter+1)
            nft = Nft(nftsCounterThisRun, nftMetadata['name'], rawAttributes, filteredAttributes,nftMetadata, nftPaths, orderedLayersPath, attributes_count, folder_path)
            nftsCreated.append(nft)
            self.nftsCreatedCounter += 1
            nftsCounterThisRun += 1
        if not self.testRarities:
            print("Created", nftsCounterThisRun, "uniques NFTs for", folder_path)
        return nftsCreated

    def ShuffleNfts(self, nftQuantity):
        total_files = len(nftQuantity)
        if not self.testRarities:
            print('Shuffling all created NFTs...', end = ' ', flush = True)
            for i in range(len(self.nfts)):
                for j in range(len(self.nfts[i])):
                    randomUniformFolder = np.random.randint(0, total_files)
                    while(nftQuantity[randomUniformFolder] <= 0):
                        randomUniformFolder = np.random.randint(0, total_files)
                    randomUniformNFT = np.random.randint(0, nftQuantity[randomUniformFolder])
                    nftAux = copy.deepcopy(self.nfts[i][j])
                    nftAux2 = copy.deepcopy(self.nfts[randomUniformFolder][randomUniformNFT])
                    self.nfts[randomUniformFolder][randomUniformNFT] = copy.deepcopy(nftAux)
                    self.nfts[randomUniformFolder][randomUniformNFT].folder_path = copy.deepcopy(nftAux2.folder_path)
                    self.nfts[randomUniformFolder][randomUniformNFT].number = copy.deepcopy(nftAux2.number)
                    self.nfts[i][j] = copy.deepcopy(nftAux2)
                    self.nfts[i][j].folder_path = copy.deepcopy(nftAux.folder_path)
                    self.nfts[i][j].number = copy.deepcopy(nftAux.number)
            print('Done.')
        return self.nfts
    
    def CreateImageAndMetadata(self, i, nftsQuantity, folder_path):
        folderMetadata = []
        folderDNA = []
        print('Creating', nftsQuantity ,'NFTs image and metadata for', folder_path,'...', end = ' ', flush = True)
        for nft in self.nfts[i]:
            nft.CreateImage()
            nft.CreateMetadata()
            folderMetadata.append(nft.metadata)
            folderDNA.append(nft.dnaPaths)
        print("Done.")
        return folderMetadata, folderDNA


    def CreateExtraJsonFiles(self, i, folder_path):
        print('Creating _metadata.json and _dna.json for', folder_path,'...', end = ' ', flush = True)
        with open(os.path.dirname(__file__) + '/../output/nfts/'+ folder_path+'_metadata.json', 'w') as jsonFile:
            json.dump(self.totalMetadata[i], jsonFile, indent = 4)

        with open(os.path.dirname(__file__) + '/../output/nfts/'+ folder_path+'_dna.json', 'w') as jsonFile:
            json.dump(self.totalDNA[i], jsonFile, indent = 4)
        print("Done.")
    
    def CreateTotalNFTInfo(self):
        print("Creating extra general NFT info...", end = ' ', flush = True)
        jsonText = []
        for nfts in self.nfts:
            for nft in nfts:
                jsonText.append({
                    "name": nft.name,
                    "attributes_count": nft.attributes_count,
                    "items": nft.filteredAttributes,
                    "folder": nft.folder_path
                })
        jsonText = sorted(jsonText, key=lambda d: d['attributes_count'])
        with open(os.path.dirname(__file__) + '/../output/nfts.json', 'w') as jsonFile:
            json.dump(jsonText, jsonFile, indent = 4)
        print("Done.")
