param location string = resourceGroup().location
param name string

resource mid 'Microsoft.ManagedIdentity/userAssignedIdentities@2018-11-30' = {
    location: location
    name: name
}

output id string = mid.id
output name string = mid.name
output clientId string = mid.properties.clientId
output objectId string = mid.properties.principalId
output tenantId string = mid.properties.tenantId
