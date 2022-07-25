param location string = resourceGroup().location
param name string
param managedIdentityId string
param storageAccountId string
param kvName string

resource kv 'Microsoft.KeyVault/vaults@2021-10-01' existing = {
    name: kvName
}

resource batchAccount 'Microsoft.Batch/batchAccounts@2022-06-01' = {
    location: location
    name: name
    identity: {
        type: 'UserAssigned'
        userAssignedIdentities: {
            '${managedIdentityId}': {}
        }
    }
    properties: {
        poolAllocationMode: 'BatchService'
        autoStorage: {
            storageAccountId: storageAccountId
            authenticationMode: 'BatchAccountManagedIdentity'
            nodeIdentityReference: {
                resourceId: managedIdentityId
            }
        }
        allowedAuthenticationModes: [
            'AAD'
            'SharedKey'
            'TaskAuthenticationToken'
        ]
    }
}

resource secret_batchLocation 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: kv
    name: 'azure-batch-location'
    properties: {
        value: location
        attributes: {
            enabled: true
        }
    }
}

resource secret_batchName 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: kv
    name: 'azure-batch-accountName'
    properties: {
        value: batchAccount.name
        attributes: {
            enabled: true
        }
    }
}

resource secret_batchKey 'Microsoft.KeyVault/vaults/secrets@2021-10-01' = {
    parent: kv
    name: 'azure-batch-accountKey'
    properties: {
        value: batchAccount.listKeys().primary
        attributes: {
            enabled: true
        }
    }
}

output id string = batchAccount.id
output name string = batchAccount.name
