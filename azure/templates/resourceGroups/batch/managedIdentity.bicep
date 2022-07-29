param location string = resourceGroup().location
param name string

resource msi 'Microsoft.ManagedIdentity/userAssignedIdentities@2018-11-30' = {
    location: location
    name: name
}

output id string = msi.id
output name string = msi.name
output clientId string = msi.properties.clientId
output objectId string = msi.properties.principalId
output tenantId string = msi.properties.tenantId
