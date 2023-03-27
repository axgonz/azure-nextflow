param location string = resourceGroup().location
param name string
param managedIdentityId string
param storageAccountName string
param objectId string
param NXFUTIL_AZ_SUB_ID string
param NXFUTIL_AZ_RG_NAME string
param NXFUTIL_AZ_KV_NAME string
param NXFUTIL_AZ_CR_NAME string
param NXFUTIL_AZ_MSI_NAME string
param NXFUTIL_AZ_MSI_ID string
param AZURE_CLIENT_ID string

resource storageAccount 'Microsoft.Storage/storageAccounts@2021-09-01' existing = {
    name: storageAccountName
}

resource appServicePlan 'Microsoft.Web/serverFarms@2020-06-01' = {
    name: name
    location: location
    kind: 'functionapp,linux'
    sku: {
        name: 'Y1'
        tier: 'Dynamic'
    }
    properties: {
        reserved: true
    }
}

resource applicationInsights 'Microsoft.Insights/components@2020-02-02' = {
    name: name
    location: location
    kind: 'web'
    properties: {
        Application_Type: 'web'
        Flow_Type: 'Redfield'
        Request_Source: 'IbizaAIExtensionEnablementBlade'
    }
}

resource functionApp 'Microsoft.Web/sites@2021-03-01' = {
    name: name
    location: location
    kind: 'functionapp,linux'
    identity: {
        type: 'UserAssigned'
        userAssignedIdentities: {
        '${managedIdentityId}': {}
        }
    }
    properties: {
        serverFarmId: appServicePlan.id
        httpsOnly: true
        siteConfig: {
            minTlsVersion: '1.2'
            appSettings: [
                {
                    name: 'AzureWebJobsStorage'
                    value: 'DefaultEndpointsProtocol=https;AccountName=${storageAccountName};EndpointSuffix=${environment().suffixes.storage};AccountKey=${storageAccount.listKeys().keys[0].value}'
                }
                {
                    name: 'WEBSITE_CONTENTAZUREFILECONNECTIONSTRING'
                    value: 'DefaultEndpointsProtocol=https;AccountName=${storageAccountName};EndpointSuffix=${environment().suffixes.storage};AccountKey=${storageAccount.listKeys().keys[0].value}'
                }
                {
                    name: 'APPINSIGHTS_INSTRUMENTATIONKEY'
                    value: applicationInsights.properties.InstrumentationKey
                }
                {
                    name: 'FUNCTIONS_EXTENSION_VERSION'
                    value: '~4'
                }
                {
                    name: 'FUNCTIONS_WORKER_RUNTIME'
                    value: 'custom'
                }              
            ]
        }
    }
}

resource functionApp_with_envVars 'Microsoft.Web/sites@2021-03-01' = {
    name: name
    location: location
    kind: 'functionapp,linux'
    identity: {
        type: 'UserAssigned'
        userAssignedIdentities: {
        '${managedIdentityId}': {}
        }
    }
    properties: {
        serverFarmId: appServicePlan.id
        httpsOnly: true
        siteConfig: {
            minTlsVersion: '1.2'
            appSettings: [
                {
                    name: 'AzureWebJobsStorage'
                    value: 'DefaultEndpointsProtocol=https;AccountName=${storageAccountName};EndpointSuffix=${environment().suffixes.storage};AccountKey=${storageAccount.listKeys().keys[0].value}'
                }
                {
                    name: 'WEBSITE_CONTENTAZUREFILECONNECTIONSTRING'
                    value: 'DefaultEndpointsProtocol=https;AccountName=${storageAccountName};EndpointSuffix=${environment().suffixes.storage};AccountKey=${storageAccount.listKeys().keys[0].value}'
                }
                {
                    name: 'APPINSIGHTS_INSTRUMENTATIONKEY'
                    value: applicationInsights.properties.InstrumentationKey
                }
                {
                    name: 'FUNCTIONS_EXTENSION_VERSION'
                    value: '~4'
                }
                {
                    name: 'FUNCTIONS_WORKER_RUNTIME'
                    value: 'custom'
                } 
                {
                    name: 'NXFUTIL_API_FQDN'
                    value: functionApp.properties.hostNames[0]
                } 
                {
                    name: 'NXFUTIL_AZ_SUB_ID'
                    value: NXFUTIL_AZ_SUB_ID
                }
                {
                    name: 'NXFUTIL_AZ_RG_NAME'
                    value: NXFUTIL_AZ_RG_NAME
                }
                {
                    name: 'NXFUTIL_AZ_KV_NAME'
                    value: NXFUTIL_AZ_KV_NAME
                }
                {
                    name: 'NXFUTIL_AZ_CR_NAME'
                    value: NXFUTIL_AZ_CR_NAME
                }               
                {
                    name: 'NXFUTIL_AZ_MSI_NAME'
                    value: NXFUTIL_AZ_MSI_NAME
                }
                {
                    name: 'NXFUTIL_AZ_MSI_ID'
                    value: NXFUTIL_AZ_MSI_ID
                }
                {
                    name: 'AZURE_CLIENT_ID'
                    value: AZURE_CLIENT_ID
                }           
            ]
        }
    }
}

resource roleDefinition_Contributor 'Microsoft.Authorization/roleDefinitions@2018-01-01-preview' existing = {
    scope: subscription()
    name: 'b24988ac-6180-42a0-ab88-20f7382dd24c'
}

resource roleAssignment 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
    scope: resourceGroup()
    name: guid(storageAccount.id, objectId, roleDefinition_Contributor.id)
    properties: {
        roleDefinitionId: roleDefinition_Contributor.id
        principalId: objectId
        principalType: 'ServicePrincipal'
    }
}

output id string = functionApp.id
output name string = functionApp.name
output fqdn string = functionApp.properties.hostNames[0]
