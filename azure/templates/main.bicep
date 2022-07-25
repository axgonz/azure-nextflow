targetScope = 'subscription'

param location string = deployment().location

var configText = loadTextContent('./main.json')
var config = json(configText)

// Get the short location and update place holders in config
// var shortLocation = config.regionPrefixLookup[location]

// Create core resource group and update the deployment
resource rg_batch 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: config.resourceGroup.nameIsAlreadyUnique ? config.resourceGroup.name : '${config.resourceGroup.name}${substring(uniqueString(config.resourceGroup.name, subscription().id, location), 0, 4)}'
  location: location
}

module dep_midBatch 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${rg_batch.name}-managedIdentity_batch'
  scope: rg_batch
  params: {
    location: location
    name: config.managedIdentity.batch.name
  }
}

module dep_midNextflow 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${rg_batch.name}-managedIdentity_nextflow'
  scope: rg_batch
  params: {
    location: location
    name: config.managedIdentity.nextflow.name
  }
}

module dep_acr 'resourceGroups/batch/containerRegistry.bicep' = {
  name: '${rg_batch.name}-containerRegistry'
  scope: rg_batch
  params: {
    location: location
    name: config.containerRegistry.nameIsAlreadyUnique ? config.containerRegistry.name : '${config.containerRegistry.name}${substring(uniqueString(config.containerRegistry.name, subscription().id, location), 0, 4)}'
  }
}

module dep_storage 'resourceGroups/batch/storageAccount.bicep' = {
  name: '${rg_batch.name}-storageAccount'
  scope: rg_batch
  params: {
    location: location
    name: config.storageAccount.nameIsAlreadyUnique ? config.storageAccount.name : '${config.storageAccount.name}${substring(uniqueString(config.storageAccount.name, subscription().id, location), 0, 4)}'
    objectId: dep_midBatch.outputs.objectId
  }
}

module dep_batch 'resourceGroups/batch/batchAccount.bicep' = {
  name: '${rg_batch.name}-batchAccount'
  scope: rg_batch
  params: {
    location: location
    name: config.batchAccount.nameIsAlreadyUnique ? config.batchAccount.name : '${config.batchAccount.name}${substring(uniqueString(config.batchAccount.name, subscription().id, location), 0, 4)}'
    managedIdentityId: dep_midBatch.outputs.id
    storageAccountId: dep_storage.outputs.id
  }
}

module dep_kv 'resourceGroups/batch/keyVault.bicep' = {
  name: '${rg_batch.name}-keyVault'
  scope: rg_batch
  params: {
    location: location
    name: config.keyVault.nameIsAlreadyUnique ? config.keyVault.name : '${config.keyVault.name}${substring(uniqueString(config.keyVault.name, subscription().id, location), 0, 4)}'
    tenantId: dep_midNextflow.outputs.tenantId
    objectId: dep_midNextflow.outputs.objectId
  }
}
