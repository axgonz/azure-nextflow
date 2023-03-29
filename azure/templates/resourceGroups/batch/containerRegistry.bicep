param location string = resourceGroup().location
param name string
param keyVaultName string

resource keyVault 'Microsoft.KeyVault/vaults@2021-10-01' existing = {
    name: keyVaultName
}

resource containerRegistry 'Microsoft.ContainerRegistry/registries@2021-09-01' = {
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
    parent: keyVault
    name: 'azure-registry-server'
    properties: {
        value: containerRegistry.properties.loginServer
        attributes: {
            enabled: true
        }
    }
}

resource secret_batchName 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: keyVault
    name: 'azure-registry-username'
    properties: {
        value: containerRegistry.listCredentials().username
        attributes: {
            enabled: true
        }
    }
}

resource secret_batchKey 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: keyVault
    name: 'azure-registry-password'
    properties: {
        value: containerRegistry.listCredentials().passwords[0].value
        attributes: {
            enabled: true
        }
    }
}

output id string = containerRegistry.id
output name string = containerRegistry.name
output server string = containerRegistry.properties.loginServer
