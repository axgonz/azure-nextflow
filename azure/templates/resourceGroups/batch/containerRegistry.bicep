param location string = resourceGroup().location
param name string
param kvName string

resource kv 'Microsoft.KeyVault/vaults@2021-10-01' existing = {
    name: kvName
}

resource acr 'Microsoft.ContainerRegistry/registries@2021-09-01' = {
    location: location
    name: name
    sku: {
        name: 'Standard'
    }
    properties: {
        adminUserEnabled: true
    }
}

resource secret_batchLocation 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: kv
    name: 'azure-registry-server'
    properties: {
        value: acr.properties.loginServer
        attributes: {
            enabled: true
        }
    }
}

resource secret_batchName 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: kv
    name: 'azure-registry-username'
    properties: {
        value: acr.listCredentials().username
        attributes: {
            enabled: true
        }
    }
}

resource secret_batchKey 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: kv
    name: 'azure-registry-password'
    properties: {
        value: acr.listCredentials().passwords[0].value
        attributes: {
            enabled: true
        }
    }
}

output id string = acr.id
output name string = acr.name
output server string = acr.properties.loginServer
