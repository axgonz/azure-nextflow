param location string = resourceGroup().location
param name string
param tenantId string
param objectId string

resource kv 'Microsoft.KeyVault/vaults@2021-10-01' = {
    location: location
    name: name
    properties: {
        sku: {
            family: 'A'
            name: 'standard'
        }
        tenantId: tenantId
        accessPolicies: [
            {
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
            }
        ]
        enabledForDeployment: true
        enabledForDiskEncryption: true
        enabledForTemplateDeployment: true
        enableSoftDelete: true
    }
}

output id string = kv.id
output name string = kv.name
output vaultUri string = kv.properties.vaultUri
