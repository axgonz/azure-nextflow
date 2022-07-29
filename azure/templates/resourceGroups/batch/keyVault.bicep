param location string = resourceGroup().location
param name string
param tenantId string
param objectIds array

resource keyVault 'Microsoft.KeyVault/vaults@2021-10-01' = {
    location: location
    name: name
    properties: {
        sku: {
            family: 'A'
            name: 'standard'
        }
        tenantId: tenantId
        accessPolicies: [for objectId in objectIds: {
                tenantId: tenantId
                objectId: objectId
                permissions: {
                    keys: []
                    secrets: [
                        'Get'
                        'List'
                    ]
                    certificates: []
                }
            }]
        enabledForDeployment: true
        enabledForDiskEncryption: true
        enabledForTemplateDeployment: true
        enableSoftDelete: true
    }
}

output id string = keyVault.id
output name string = keyVault.name
output vaultUri string = keyVault.properties.vaultUri
