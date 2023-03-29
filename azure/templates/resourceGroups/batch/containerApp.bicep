param location string = resourceGroup().location
param name string
param storageAccountName string
param managedIdentityId string
param managedIdentityClientId string

param NXFUTIL_API_FQDN string = '<not_defined>'
param NXFUTIL_AZ_CR_NAME string = '<not_defined>'
param NXFUTIL_AZ_KV_NAME string = '<not_defined>'
param NXFUTIL_AZ_MSI_NAME string = '<not_defined>'
param NXFUTIL_AZ_MSI_ID string = '<not_defined>'

resource managedEnvironment 'Microsoft.App/managedEnvironments@2022-10-01' = {
  location: location
  name: name
  sku: {
    name: 'Consumption'
  }
  properties: {}
}

resource containerApp 'Microsoft.App/containerApps@2022-10-01' = {
  name: name
  location: location
  identity: {
    type: 'UserAssigned'
    userAssignedIdentities: {
      '${managedIdentityId}': {}
    }
  }
  properties: {
    managedEnvironmentId: managedEnvironment.id
    configuration: {
      registries: [
        {
          identity: managedIdentityId
          server: 'crodxx.azurecr.io'
          username: 'crodxx'
        }
      ]
      ingress: {
        targetPort: 3000
        external: true
        corsPolicy: {
          allowCredentials: true
          maxAge: 0
          allowedHeaders: [
            '*'
          ]
          allowedMethods: [
            '*'
          ]
          allowedOrigins: [
            '*'
          ]
        }
      }
    }
    template: {
      scale: {
        minReplicas: 1
        maxReplicas: 4
      }
      containers: [
        {
          name: name
          image: 'crodxx.azurecr.io/default/nxfutil:latest'
          env: [
            {
              name: 'NXFUTIL_AZ_SUB_ID'
              value: subscription().subscriptionId
            }
            {
              name: 'NXFUTIL_AZ_RG_NAME'
              value: resourceGroup().name
            }
            {
              name: 'NXFUTIL_AZ_ST_NAME'
              value: storageAccountName
            }
            {
              name: 'NXFUTIL_API_FQDN'
              value: NXFUTIL_API_FQDN
            }                
            {
              name: 'NXFUTIL_AZ_KV_NAME'
              value: NXFUTIL_AZ_KV_NAME
            }
            {
              name: 'NXFUTIL_AZ_CR_NAME'
              value: NXFUTIL_AZ_CR_NAME
            }
            {
              name: 'NXFUTIL_AZ_MSI_NAME'
              value: NXFUTIL_AZ_MSI_NAME
            }
            {
              name: 'NXFUTIL_AZ_MSI_ID'
              value: NXFUTIL_AZ_MSI_ID
            }
            {
              name: 'AZURE_CLIENT_ID'
              value: managedIdentityClientId
            }
          ]
        }
      ]
    }
  }
}

output id string = containerApp.id
output name string = containerApp.name
output fqdn string = containerApp.properties.latestRevisionFqdn 
