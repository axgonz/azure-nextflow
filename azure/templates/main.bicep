targetScope = 'resourceGroup'

param location string = resourceGroup().location
param deploymentPrincipalObjectId string = 'null'

var configText = loadTextContent('./main.json')
var config = json(configText)

// Get the short location and update place holders in config
//var shortLocation = config.regionPrefixLookup[location]

module dep_msiBatchAccount 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${resourceGroup().name}-managedIdentity_batch'
  params: {
    location: location
    name: config.managedIdentity.batchAccount.name
  }
}

module dep_msiFunctionApp 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${resourceGroup().name}-managedIdentity_func'
  params: {
    location: location
    name: config.managedIdentity.functionApp.name
  }
}

module dep_msiNextflow 'resourceGroups/batch/managedIdentity.bicep' = {
  name: '${resourceGroup().name}-managedIdentity_nextflow'
  params: {
    location: location
    name: config.managedIdentity.containerInstance.name
  }
}

module dep_storageAccount 'resourceGroups/batch/storageAccount.bicep' = {
  name: '${resourceGroup().name}-storageAccount'
  params: {
    location: location
    name: config.storageAccount.nameIsAlreadyUnique ? config.storageAccount.name : '${config.storageAccount.name}${substring(uniqueString(config.storageAccount.name, subscription().subscriptionId, resourceGroup().name, location), 0, 4)}'
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_containerRegistry 'resourceGroups/batch/containerRegistry.bicep' = {
  name: '${resourceGroup().name}-containerRegistry'
  params: {
    location: location
    name: config.containerRegistry.nameIsAlreadyUnique ? config.containerRegistry.name : '${config.containerRegistry.name}${substring(uniqueString(config.containerRegistry.name, subscription().subscriptionId, resourceGroup().name, location), 0, 4)}'
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_batchAccount 'resourceGroups/batch/batchAccount.bicep' = {
  name: '${resourceGroup().name}-batchAccount'
  params: {
    location: location
    name: config.batchAccount.nameIsAlreadyUnique ? config.batchAccount.name : '${config.batchAccount.name}${substring(uniqueString(config.batchAccount.name, subscription().subscriptionId, resourceGroup().name, location), 0, 4)}'
    managedIdentityId: dep_msiBatchAccount.outputs.id
    storageAccountId: dep_storageAccount.outputs.id
    keyVaultName: dep_keyVault.outputs.name
  }
}

module dep_functionApp 'resourceGroups/batch/functionApp.bicep' = if (config.deployContainerAppInsteadOfFunctionApp == false) {
  name: '${resourceGroup().name}-functionApp'
  params: {
    location: location
    name: config.functionApp.nameIsAlreadyUnique ? config.functionApp.name : '${config.functionApp.name}${substring(uniqueString(config.functionApp.name, subscription().subscriptionId, resourceGroup().name, location), 0, 4)}'
    storageAccountName: dep_storageAccount.outputs.name
    managedIdentityId: dep_msiFunctionApp.outputs.id
    managedIdentityClientId: dep_msiFunctionApp.outputs.clientId
    NXFUTIL_AZ_KV_NAME: dep_keyVault.outputs.name
    NXFUTIL_AZ_CR_NAME: dep_containerRegistry.outputs.name
    NXFUTIL_AZ_MSI_NAME: dep_msiNextflow.outputs.name
    NXFUTIL_AZ_MSI_ID: dep_msiNextflow.outputs.clientId
  }
}

module dep_containerApp 'resourceGroups/batch/containerAppWrapper.bicep' = if (config.deployContainerAppInsteadOfFunctionApp == true) {
  name: '${resourceGroup().name}-containerApp'
  params: {
    location: location
    name: config.containerApp.nameIsAlreadyUnique ? config.containerApp.name : '${config.containerApp.name}${substring(uniqueString(config.containerApp.name, subscription().subscriptionId, resourceGroup().name, location), 0, 4)}'
    storageAccountName: dep_storageAccount.outputs.name
    containerRegistryName: dep_containerRegistry.outputs.name
    managedIdentityId: dep_msiFunctionApp.outputs.id
    managedIdentityClientId: dep_msiFunctionApp.outputs.clientId
    NXFUTIL_AZ_KV_NAME: dep_keyVault.outputs.name
    NXFUTIL_AZ_MSI_NAME: dep_msiNextflow.outputs.name
    NXFUTIL_AZ_MSI_ID: dep_msiNextflow.outputs.clientId
  }
  dependsOn: [
    dep_permissions
  ]
}

module dep_keyVault 'resourceGroups/batch/keyVault.bicep' = {
  name: '${resourceGroup().name}-keyVault'
  params: {
    location: location
    name: config.keyVault.nameIsAlreadyUnique ? config.keyVault.name : '${config.keyVault.name}${substring(uniqueString(config.keyVault.name, subscription().subscriptionId, resourceGroup().name, location), 0, 4)}'
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

module dep_permissions 'resourceGroups/batch/permissions.bicep' = {
  name: '${resourceGroup().name}-permissions'
  params: {
    functionAppMsi_objectId: dep_msiFunctionApp.outputs.objectId
    nextflowMsi_objectId: dep_msiNextflow.outputs.objectId
    batchMsi_objectId: dep_msiBatchAccount.outputs.objectId
    storageAccountName: dep_storageAccount.outputs.name
    containerRegistryName: dep_containerRegistry.outputs.name
  }
}

output functionAppName string = config.deployContainerAppInsteadOfFunctionApp ? '' : dep_functionApp.outputs.name
output keyVaultName string = dep_keyVault.outputs.name

output NXFUTIL_AZ_SUB_ID string = subscription().subscriptionId
output NXFUTIL_AZ_RG_NAME string = resourceGroup().name
output NXFUTIL_AZ_KV_NAME string = dep_keyVault.outputs.name
output NXFUTIL_AZ_CR_NAME string = dep_containerRegistry.outputs.name
output NXFUTIL_AZ_MSI_ID string = dep_msiNextflow.outputs.clientId
output AZURE_CLIENT_ID string = dep_msiFunctionApp.outputs.clientId
