# Azure Nextflow

This repository contains sample only code to demonstrate how secrets can be marshaled from Key Vault into a Container Instance running Nextflow for the purpose of dispatching Nextflow pipelines on Azure Batch.

## Overview

Deploying the Azure resources supporting this sample is left to you.

It is assumed that this infrastructure would be deployed through a DevOps pipeline or GitHub workflow. 

## Set up

The following uses the provided GitHub workflows to build and deploy the sample. See Annex below for manual set up instructions.

### Azure resources

1. Create `AZURE_CREDENTIALS` GitHub Secret; details in [Azure Docs](https://docs.microsoft.com/en-us/azure/developer/github/connect-from-azure?tabs=azure-cli%2Cwindows#create-a-service-principal-and-add-it-as-a-github-secret).

1. Run `Build and deploy Azure templates` GitHub Action and/or add automatic trigger to workflow.

### Docker images

1. Create `AZURE_CONTAINERREGISTRY_PASSWORD` GitHub Secret; details in [Azure Docs](https://docs.microsoft.com/en-us/azure/container-registry/container-registry-authentication?tabs=azure-cli#admin-account)

1. Run `Build and push Docker images` GitHub Action and/or add automatic trigger to workflow.

### Function packages

1. Create `AZURE_FUNCTIONAPP_PUBLISH_PROFILE` GitHub Secret; details in [Azure Docs](https://docs.microsoft.com/en-us/azure/azure-functions/functions-how-to-github-actions?tabs=python). 

1. Run `Build and publish Python functions` GitHub Action and/or add automatic trigger to workflow.

## Validation

To validate set up was successful, trigger the Azure Function using its webhook.

``` bash
az_funcAppName="myFuncAppName"

curl https://$az_funcAppName.azurewebsites.net/api/nxfutil?
```

## Usage

When the Function App is triggered it will create a new nxfutil Container Instance. See Annex for nxfutil details.

The http trigger currently accepts 3 (optional) arguments.
- A URI to a Nextflow `config` file. 
- A URI to a Nextflow `pipeline` file.
- A URI to a Nextflow `parameters` file.

``` bash
az_funcAppName="nxfutil-py"

nxf_configUri="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/nextflow.config"
nxf_pipelineUri="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/pipeline.nf"
nxf_parametersUri="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/parameters.json"

curl "https://$az_funcAppName.azurewebsites.net/api/nxfutil?config=$nxf_configUri&pipeline=$nxf_pipelineUri&parameters=$nxf_parametersUri"
```

## Annexes

### [Azure infrastructure](./docs/AzureInfrastructure.md)

### [Azure Functions - Python](./azure/functions/python/README.md)

### [nxfutil](./docs/nxfutil.md)

### [Data upload](./docs/DataUpload.md)

### [Manual set up](./docs/ManualSetup.md)
