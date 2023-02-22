targetScope = 'subscription'

param location string = deployment().location
param resourceGroupName string
param deploymentPrincipalObjectId string = 'null'

var configText = loadTextContent('./main.json')
var config = json(configText)

// Get the short location and update place holders in config
//var shortLocation = config.regionPrefixLookup[location]

// Create core resource group and update the deployment
resource rg_batch 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: resourceGroupName
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
    name: config.containerRegistry.nameIsAlreadyUnique ? config.containerRegistry.name : '${config.containerRegistry.name}${substring(uniqueString(config.containerRegistry.name, subscription().subscriptionId, rg_batch.name, location), 0, 4)}'
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_storageAccount 'resourceGroups/batch/storageAccount.bicep' = {
  name: '${rg_batch.name}-storageAccount'
  scope: rg_batch
  params: {
    location: location
    name: config.storageAccount.nameIsAlreadyUnique ? config.storageAccount.name : '${config.storageAccount.name}${substring(uniqueString(config.storageAccount.name, subscription().subscriptionId, rg_batch.name, location), 0, 4)}'
    batchMsi_objectId: dep_msiBatchAccount.outputs.objectId
    nextflowMsi_objectId: dep_msiNextflow.outputs.objectId
    funcMsi_objectId: dep_msiFunctionApp.outputs.objectId
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_batchAccount 'resourceGroups/batch/batchAccount.bicep' = {
  name: '${rg_batch.name}-batchAccount'
  scope: rg_batch
  params: {
    location: location
    name: config.batchAccount.nameIsAlreadyUnique ? config.batchAccount.name : '${config.batchAccount.name}${substring(uniqueString(config.batchAccount.name, subscription().subscriptionId, rg_batch.name, location), 0, 4)}'
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
    name: config.functionApp.nameIsAlreadyUnique ? config.functionApp.name : '${config.functionApp.name}${substring(uniqueString(config.functionApp.name, subscription().subscriptionId, rg_batch.name, location), 0, 4)}'
    managedIdentityId: dep_msiFunctionApp.outputs.id
    storageAccountName: dep_storageAccount.outputs.name
    objectId: dep_msiFunctionApp.outputs.objectId
    NXFUTIL_AZ_SUB_ID: subscription().subscriptionId
    NXFUTIL_AZ_RG_NAME: rg_batch.name
    NXFUTIL_AZ_KV_NAME: dep_keyVault.outputs.name
    NXFUTIL_AZ_CR_NAME: dep_containerRegistry.outputs.name
    NXFUTIL_AZ_MSI_NAME: dep_msiNextflow.outputs.name
    NXFUTIL_AZ_MSI_ID: dep_msiNextflow.outputs.clientId
    AZURE_CLIENT_ID: dep_msiFunctionApp.outputs.clientId
  }
}

module dep_keyVault 'resourceGroups/batch/keyVault.bicep' = {
  name: '${rg_batch.name}-keyVault'
  scope: rg_batch
  params: {
    location: location
    name: config.keyVault.nameIsAlreadyUnique ? config.keyVault.name : '${config.keyVault.name}${substring(uniqueString(config.keyVault.name, subscription().subscriptionId, rg_batch.name, location), 0, 4)}'
    tenantId: dep_msiNextflow.outputs.tenantId
    objectIds: deploymentPrincipalObjectId == 'null' ? [
      dep_msiFunctionApp.outputs.objectId
      dep_msiNextflow.outputs.objectId
    ] : [
      dep_msiFunctionApp.outputs.objectId
      dep_msiNextflow.outputs.objectId
      deploymentPrincipalObjectId
    ]
  }
}

output functionAppId string = dep_functionApp.outputs.id
output functionAppName string = dep_functionApp.outputs.name
output keyVaultId string = dep_keyVault.outputs.id
output keyVaultName string = dep_keyVault.outputs.name

output NXFUTIL_AZ_SUB_ID string = subscription().subscriptionId
output NXFUTIL_AZ_RG_NAME string = rg_batch.name
output NXFUTIL_AZ_KV_NAME string = dep_keyVault.outputs.name
output NXFUTIL_AZ_CR_NAME string = dep_containerRegistry.outputs.name
output NXFUTIL_AZ_MSI_ID string = dep_msiNextflow.outputs.clientId
output AZURE_CLIENT_ID string = dep_msiFunctionApp.outputs.clientId
