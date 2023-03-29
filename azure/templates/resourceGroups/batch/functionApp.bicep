param location string = resourceGroup().location
param name string
param storageAccountName string
param managedIdentityId string
param managedIdentityClientId string 

param NXFUTIL_API_FQDN string = '<not_defined>'
param NXFUTIL_AZ_CR_NAME string = '<not_defined>'
param NXFUTIL_AZ_KV_NAME string = '<not_defined>'
param NXFUTIL_AZ_MSI_NAME string = '<not_defined>'
param NXFUTIL_AZ_MSI_ID string = '<not_defined>'

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
                {
                    name: 'NXFUTIL_AZ_SUB_ID'
                    value: subscription().subscriptionId
                }
                {
                    name: 'NXFUTIL_AZ_RG_NAME'
                    value: resourceGroup().name
                }
                {
                    name: 'NXFUTIL_AZ_ST_NAME'
                    value: storageAccountName
                }      
                {
                    name: 'NXFUTIL_API_FQDN'
                    value: NXFUTIL_API_FQDN
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
                    value: managedIdentityClientId
                }           
            ]
        }
    }
}

output id string = functionApp.id
output name string = functionApp.name
output fqdn string = functionApp.properties.hostNames[0]
