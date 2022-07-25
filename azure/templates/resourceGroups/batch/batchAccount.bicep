param location string = resourceGroup().location
param name string
param managedIdentityId string
param storageAccountId string

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

output id string = batchAccount.id
output name string = batchAccount.name
