# Azure Nextflow

This repository contains sample only code to demonstrate how secrets can be marshaled from Key Vault into a Container Instance running Nextflow for the purpose of dispatching Nextflow pipelines on Azure Batch.

## Overview

Deploying the Azure infrastructure supporting this sample is left to you.

It is assumed that this infrastructure would be deployed through a DevOps pipeline of GitHub workflow. 

This sample will be eventually expanded to include Bicep templates for capturing the state of required Azure resources and pipeline/workflow files for the deployment.

## Set up

As per above, it is assumed (for now) that the following Azure resources are already deployed.

### Key Vault

With the following secrets created:

- azure-registry-server
- azure-registry-userName
- azure-registry-password
- azure-batch-location
- azure-batch-accountName
- azure-batch-accountKey
- azure-storage-accountName
- azure_storage_accountName
- azure-storage-accountKey
- azure_storage_accountKey

### Container Registry

- Nextflow image built and pushed to registry.
- Ubuntu image built and pushed to registry.
- Admin user enabled and keys stored in Key Vault.

### Storage Account

- Storage blob with 'batch' container created.
- Storage files with 'batchsmb' share created.
- Keys enabled and stored in Key Vault.

### Batch Account

- Pool allocation mode set to "Batch service".
- Connected to above Storage Account using a Managed Identity.
- Keys enabled and stored in Key Vault.

### Container Instance

- Configured using the nextflow image.
- System Assigned Managed Identity enabled with read access granted in Key Vault.
- Override the "AZ_KEY_VAULT_NAME" environment variable with the name of the Key Vault resource.
- (optional) Override the CMD with one similar to that below to execute a different Nextflow pipeline.
    ``` bash
    cd /.nextflow && \
    ./nxfutil -c "https://<...>/nextflow.config" \
        -p "https://<...>/pipeline.nf" \
        -a "https://<...>/parameters.json"
    ``` 

## Usage

Once deployed the Container Instance will start and execute the default command of `./nxfutil` or that provided when the Container Instance was deployed.

Once called, nxfutil will download the default or provided Nextflow files and parse the "nextflow.config" to create a list of secrets it will need to retrieve from Key Vault. At this time, it will also expand and replace any `exParams` parameters with their values (also retrieved from Key Vault).

Once the config file has been parsed nxfutil will show the resultant Nextflow config by running `nextflow config` and will finally offload to nextflow by running `nextflow run` specifying the pipeline and parameters files.

It is intended that the next iteration of nxfutil will integrate with Nextflow Tower so that launched jobs can be monitored remotely. It is also possible that the utility will be re-written using nodejs, dotNet and/or Rust.