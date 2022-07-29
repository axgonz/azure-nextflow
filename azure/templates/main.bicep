targetScope = 'subscription'

param location string = deployment().location

var configText = loadTextContent('./main.json')
var config = json(configText)

// Get the short location and update place holders in config
//var shortLocation = config.regionPrefixLookup[location]

// Create core resource group and update the deployment
resource rg_batch 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: config.resourceGroup.nameIsAlreadyUnique ? config.resourceGroup.name : '${config.resourceGroup.name}${substring(uniqueString(config.resourceGroup.name, subscription().id, location), 0, 4)}'
  location: location
}

module dep_msiBatchAccount 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${rg_batch.name}-managedIdentity_batch'
  scope: rg_batch
  params: {
    location: location
    name: config.managedIdentity.batchAccount.name
  }
}

module dep_msiFunctionApp 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${rg_batch.name}-managedIdentity_func'
  scope: rg_batch
  params: {
    location: location
    name: config.managedIdentity.functionApp.name
  }
}

module dep_msiNextflow 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${rg_batch.name}-managedIdentity_nextflow'
  scope: rg_batch
  params: {
    location: location
    name: config.managedIdentity.containerInstance.name
  }
}

module dep_containerRegistry 'resourceGroups/batch/containerRegistry.bicep' = {
  name: '${rg_batch.name}-containerRegistry'
  scope: rg_batch
  params: {
    location: location
    name: config.containerRegistry.nameIsAlreadyUnique ? config.containerRegistry.name : '${config.containerRegistry.name}${substring(uniqueString(config.containerRegistry.name, subscription().id, location), 0, 4)}'
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_storageAccount 'resourceGroups/batch/storageAccount.bicep' = {
  name: '${rg_batch.name}-storageAccount'
  scope: rg_batch
  params: {
    location: location
    name: config.storageAccount.nameIsAlreadyUnique ? config.storageAccount.name : '${config.storageAccount.name}${substring(uniqueString(config.storageAccount.name, subscription().id, location), 0, 4)}'
    objectId: dep_msiBatchAccount.outputs.objectId
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_batchAccount 'resourceGroups/batch/batchAccount.bicep' = {
  name: '${rg_batch.name}-batchAccount'
  scope: rg_batch
  params: {
    location: location
    name: config.batchAccount.nameIsAlreadyUnique ? config.batchAccount.name : '${config.batchAccount.name}${substring(uniqueString(config.batchAccount.name, subscription().id, location), 0, 4)}'
    managedIdentityId: dep_msiBatchAccount.outputs.id
    storageAccountId: dep_storageAccount.outputs.id
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_functionApp 'resourceGroups/batch/functionApp.bicep' = {
  name: '${rg_batch.name}-functionApp'
  scope: rg_batch
  params: {
    location: location
    name: config.functionApp.nameIsAlreadyUnique ? config.functionApp.name : '${config.functionApp.name}${substring(uniqueString(config.functionApp.name, subscription().id, location), 0, 4)}'
    managedIdentityId: dep_msiFunctionApp.outputs.id
    storageAccountName: dep_storageAccount.outputs.name
    objectId: dep_msiFunctionApp.outputs.objectId
  }
}

module dep_keyVault 'resourceGroups/batch/keyVault.bicep' = {
  name: '${rg_batch.name}-keyVault'
  scope: rg_batch
  params: {
    location: location
    name: config.keyVault.nameIsAlreadyUnique ? config.keyVault.name : '${config.keyVault.name}${substring(uniqueString(config.keyVault.name, subscription().id, location), 0, 4)}'
    tenantId: dep_msiNextflow.outputs.tenantId
    objectIds: [
      dep_msiFunctionApp.outputs.objectId
      dep_msiNextflow.outputs.objectId
    ]
  }
}

output functionAppId string = dep_functionApp.outputs.id
output functionAppName string = dep_functionApp.outputs.name

output NXFUTIL_AZ_SUB_ID string = subscription().id
output NXFUTIL_AZ_RG_NAME string = rg_batch.name
output NXFUTIL_AZ_KV_NAME string = dep_keyVault.outputs.name
output NXFUTIL_AZ_CR_NAME string = dep_containerRegistry.outputs.name
output NXFUTIL_AZ_MSI_ID string = dep_msiNextflow.outputs.clientId
output AZURE_CLIENT_ID string = dep_msiFunctionApp.outputs.clientId