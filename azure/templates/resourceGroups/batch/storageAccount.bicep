param location string = resourceGroup().location
param name string
param batchMsi_objectId string
param nextflowMsi_objectId string
param funcMsi_objectId string
param keyVaultName string

resource keyVault 'Microsoft.KeyVault/vaults@2021-10-01' existing = {
    name: keyVaultName
}

resource storageAccount 'Microsoft.Storage/storageAccounts@2021-09-01' = {
    location: location
    name: name
    kind: 'StorageV2'
    sku: {
        name: 'Standard_GRS'
    }
    properties: {
        minimumTlsVersion: 'TLS1_2'
        allowBlobPublicAccess: true
        allowSharedKeyAccess: true
        supportsHttpsTrafficOnly: true
    }
}

resource roleDefinition_Contributor 'Microsoft.Authorization/roleDefinitions@2018-01-01-preview' existing = {
    scope: subscription()
    name: 'b24988ac-6180-42a0-ab88-20f7382dd24c'
}

resource roleDefinition_StorageQueueDataContributor 'Microsoft.Authorization/roleDefinitions@2018-01-01-preview' existing = {
    scope: subscription()
    name: '974c5e8b-45b9-4653-ba55-5f855dd0fb88'
}

resource batchMsi_roleAssignment_Cont 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
    scope: storageAccount
    name: guid(storageAccount.id, batchMsi_objectId, roleDefinition_Contributor.id)
    properties: {
        roleDefinitionId: roleDefinition_Contributor.id
        principalId: batchMsi_objectId
        principalType: 'ServicePrincipal'
    }
}

resource nextflowMsi_roleAssignment_QueueCont 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
    scope: storageAccount
    name: guid(storageAccount.id, nextflowMsi_objectId, roleDefinition_StorageQueueDataContributor.id)
    properties: {
        roleDefinitionId: roleDefinition_StorageQueueDataContributor.id
        principalId: nextflowMsi_objectId
        principalType: 'ServicePrincipal'
    }
}

resource funcMsi_roleAssignment_QueueCont 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
    scope: storageAccount
    name: guid(storageAccount.id, funcMsi_objectId, roleDefinition_StorageQueueDataContributor.id)
    properties: {
        roleDefinitionId: roleDefinition_StorageQueueDataContributor.id
        principalId: funcMsi_objectId
        principalType: 'ServicePrincipal'
    }
}

resource serviceBlob 'Microsoft.Storage/storageAccounts/blobServices@2021-09-01' = {
    parent: storageAccount
    name: 'default'
    properties: {
        containerDeleteRetentionPolicy: {
            enabled: true
            days: 7
            allowPermanentDelete: false
        }
    }
}

resource serviceFile 'Microsoft.Storage/storageAccounts/fileServices@2021-09-01' = {
    parent: storageAccount
    name: 'default'
    properties: {
        shareDeleteRetentionPolicy: {
            enabled: true
            days: 7
            allowPermanentDelete: false
        }
    }
}

resource blobContainer 'Microsoft.Storage/storageAccounts/blobServices/containers@2021-09-01' = {
    parent: serviceBlob
    name: 'batch'
}

resource fileShare 'Microsoft.Storage/storageAccounts/fileServices/shares@2021-09-01' = {
    parent: serviceFile
    name: 'batchsmb'
}

resource secret_storageName 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: keyVault
    name: 'azure-storage-accountName'
    properties: {
        value: storageAccount.name
        attributes: {
            enabled: true
        }
    }
}

resource secret_storageKey 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: keyVault
    name: 'azure-storage-accountKey'
    properties: {
        value: storageAccount.listKeys().keys[0].value
        attributes: {
            enabled: true
        }
    }
}

output id string = storageAccount.id
output name string = storageAccount.name
