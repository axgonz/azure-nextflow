param location string = resourceGroup().location
param name string

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

output id string = acr.id
output name string = acr.name
output server string = acr.properties.loginServer
