param storageAccountName string
param containerRegistryName string
param batchMsi_objectId string
param nextflowMsi_objectId string
param functionAppMsi_objectId string

/*
  GitHubActions:
  - owner -> resource group
  - pull image -> cr
  - (read KV secrets)

  funcMsi actions:
  - create ci -> resource group
  - delete ci -> resource group
  - read messages -> storage queue
  - pull image -> cr
  - (no KV secrets needed)

  nextflowMsi actions:
  - delete ci -> resource group
  - create queue -> storage queue
  - send messages -> storage queue
  - pull image -> cr
  - (read KV secrets)

  batchMsi actions:
  - storage account contributor
  - (read KV secrets)
*/

resource storageAccount 'Microsoft.Storage/storageAccounts@2021-09-01' existing = {
  name: storageAccountName
}

resource containerRegistry 'Microsoft.ContainerRegistry/registries@2021-09-01' existing = {
  name: containerRegistryName
}

// Contributor role
resource roleDefinition_Contributor 'Microsoft.Authorization/roleDefinitions@2018-01-01-preview' existing = {
  scope: subscription()
  name: 'b24988ac-6180-42a0-ab88-20f7382dd24c'
}

resource roleAssignment_RG_FuncMsi_Contributor 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
  scope: resourceGroup()
  name: guid(resourceGroup().id, functionAppMsi_objectId, roleDefinition_Contributor.id)
  properties: {
    roleDefinitionId: roleDefinition_Contributor.id
    principalId: functionAppMsi_objectId
    principalType: 'ServicePrincipal'
  }
}

resource roleAssignment_RG_NextflowMsi_Contributor 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
  scope: resourceGroup()
  name: guid(resourceGroup().id, nextflowMsi_objectId, roleDefinition_Contributor.id)
  properties: {
    roleDefinitionId: roleDefinition_Contributor.id
    principalId: nextflowMsi_objectId
    principalType: 'ServicePrincipal'
  }
}

resource roleAssignment_ST_BatchMsi_Contributor 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
  scope: storageAccount
  name: guid(storageAccount.id, batchMsi_objectId, roleDefinition_Contributor.id)
  properties: {
      roleDefinitionId: roleDefinition_Contributor.id
      principalId: batchMsi_objectId
      principalType: 'ServicePrincipal'
  }
}

// ArcPull role 
resource roleDefinition_AcrPull 'Microsoft.Authorization/roleDefinitions@2018-01-01-preview' existing = {
  scope: subscription()
  name: '7f951dda-4ed3-4680-a7ca-43fe172d538d'
}   

resource roleAssignment_CR_FuncMsi_AcrPull 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
  scope: containerRegistry
  name: guid(containerRegistry.id, functionAppMsi_objectId, roleDefinition_AcrPull.id)
  properties: {
      roleDefinitionId: roleDefinition_AcrPull.id
      principalId: functionAppMsi_objectId
      principalType: 'ServicePrincipal'
  }
}

resource roleAssignment_CR_NextflowMsi_AcrPull 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
  scope: containerRegistry
  name: guid(containerRegistry.id, nextflowMsi_objectId, roleDefinition_AcrPull.id)
  properties: {
      roleDefinitionId: roleDefinition_AcrPull.id
      principalId: nextflowMsi_objectId
      principalType: 'ServicePrincipal'
  }
}

// StorageQueueDataContributor role
resource roleDefinition_StorageQueueDataContributor 'Microsoft.Authorization/roleDefinitions@2018-01-01-preview' existing = {
  scope: subscription()
  name: '974c5e8b-45b9-4653-ba55-5f855dd0fb88'
}

resource roleAssignment_ST_FuncMsi_StorageQueueDataContributor 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
  scope: storageAccount
  name: guid(storageAccount.id, functionAppMsi_objectId, roleDefinition_StorageQueueDataContributor.id)
  properties: {
      roleDefinitionId: roleDefinition_StorageQueueDataContributor.id
      principalId: functionAppMsi_objectId
      principalType: 'ServicePrincipal'
  }
}

resource roleAssignment_ST_NextflowMsi_StorageQueueDataContributor 'Microsoft.Authorization/roleAssignments@2020-04-01-preview' = {
  scope: storageAccount
  name: guid(storageAccount.id, nextflowMsi_objectId, roleDefinition_StorageQueueDataContributor.id)
  properties: {
      roleDefinitionId: roleDefinition_StorageQueueDataContributor.id
      principalId: nextflowMsi_objectId
      principalType: 'ServicePrincipal'
  }
}
