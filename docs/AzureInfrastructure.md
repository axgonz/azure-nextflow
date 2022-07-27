# Annex - Azure infrastructure

## Key Vault

With the following secrets created:

- azure-registry-server
- azure-registry-username
- azure-registry-password
- azure-batch-location
- azure-batch-accountName
- azure-batch-accountKey
- azure-storage-accountName
- azure_storage_accountName
- azure-storage-accountKey
- azure_storage_accountKey

## Container Registry

- Nextflow image built and pushed to registry.
- Ubuntu image built and pushed to registry.
- Admin user enabled and keys stored in Key Vault.

## Storage Account

- Storage blob with 'batch' container created.
- Storage files with 'batchsmb' share created.
- Keys enabled and stored in Key Vault.

## Managed Identities

- Create a Managed Identity for Batch Account 'batchmid'.
- Create a Managed Identity for nextflow Container Instance(s) 'nextflowmid'

## Batch Account

- Pool allocation mode set to "Batch service".
- Connected to above Storage Account using User Assigned Managed Identity.
- Keys enabled and stored in Key Vault.

## Container Instance

- Configured using the nextflow image.
- Read access granted in Key Vault using User Assigned Managed Identity.
- Override the "AZ_KEY_VAULT_NAME" environment variable with the name of the Key Vault resource.
- Override the "AZURE_CLIENT_ID" environment variable with the client id of the nextflowmid Managed Identity resource.
- (optional) Override the CMD with one similar to that below to execute a different Nextflow pipeline.
    ``` bash
    cd /.nextflow && \
    ./nxfutil -c "https://<...>/nextflow.config" \
        -p "https://<...>/pipeline.nf" \
        -a "https://<...>/parameters.json"
    ``` 
