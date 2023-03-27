param location string = resourceGroup().location
param name string
param keyVaultName string
param nextflowMsi_objectId string
param funcMsi_objectId string

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

resource roleDefinition_AcrPull 'Microsoft.Authorization/roleDefinitions@2018-01-01-preview' existing = {
    scope: subscription()
    name: '7f951dda-4ed3-4680-a7ca-43fe172d538d'
}   

resource nextflowMsi_roleAssignment_ArcPull 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
    scope: containerRegistry
    name: guid(containerRegistry.id, nextflowMsi_objectId, roleDefinition_AcrPull.id)
    properties: {
        roleDefinitionId: roleDefinition_AcrPull.id
        principalId: nextflowMsi_objectId
        principalType: 'ServicePrincipal'
    }
}

resource funcMsi_roleAssignment_ArcPull 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
    scope: containerRegistry
    name: guid(containerRegistry.id, funcMsi_objectId, roleDefinition_AcrPull.id)
    properties: {
        roleDefinitionId: roleDefinition_AcrPull.id
        principalId: funcMsi_objectId
        principalType: 'ServicePrincipal'
    }
}

output id string = containerRegistry.id
output name string = containerRegistry.name
output server string = containerRegistry.properties.loginServer
