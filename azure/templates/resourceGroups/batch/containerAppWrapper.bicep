param location string = resourceGroup().location
param name string
param storageAccountName string
param managedIdentityId string
param managedIdentityClientId string

param NXFUTIL_AZ_CR_NAME string
param NXFUTIL_AZ_KV_NAME string
param NXFUTIL_AZ_MSI_NAME string
param NXFUTIL_AZ_MSI_ID string

module dep_containerApp 'containerApp.bicep' = {
  name: '${resourceGroup().name}-containerApp-deploymentFirstPass'
  params: {
    location: location
    name: name
    storageAccountName: storageAccountName
    managedIdentityId: managedIdentityId
    managedIdentityClientId: managedIdentityClientId
  }
}

module dep_containerApp_envVars 'containerApp.bicep' = {
  name: '${resourceGroup().name}-containerApp-deploymentSecondPass'
  params: {
    location: location
    name: name
    storageAccountName: storageAccountName
    managedIdentityId: managedIdentityId
    managedIdentityClientId: managedIdentityClientId
    NXFUTIL_API_FQDN: dep_containerApp.outputs.fqdn
    NXFUTIL_AZ_CR_NAME: NXFUTIL_AZ_CR_NAME
    NXFUTIL_AZ_KV_NAME: NXFUTIL_AZ_KV_NAME
    NXFUTIL_AZ_MSI_NAME: NXFUTIL_AZ_MSI_NAME
    NXFUTIL_AZ_MSI_ID: NXFUTIL_AZ_MSI_ID
  }
}
