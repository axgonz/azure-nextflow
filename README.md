# Azure Nextflow

This repository contains sample only code to demonstrate how secrets can be marshaled from Key Vault into a Container Instance running Nextflow for the purpose of dispatching Nextflow pipelines on Azure Batch.

## Overview

Deploying the Azure resources supporting this sample is left to you.

It is assumed that this infrastructure would be deployed using GitHub workflows and maintained examples are provided. Stale examples for DevOps pipelines are provided but are not working since moving to rust and will not be maintained.

It is assumed that regardless of deployment method, GitHub is used for hosting the forked repository.

## Pre-requisites

1. Fork this repository on GitHub.

1. Create a Service Principal for connecting to Azure. 

    ``` bash
    az_subId="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

    az ad sp create-for-rbac --name "DeploymentConnection" --role owner --scopes /subscriptions/$az_subId--sdk-auth
    ```

1. (optional) Access can be granted at the resource group scope instead of the subscription scope if the resource group is created before running the deployment workflow/pipeline.
    
    ``` bash
    az_subId="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
    az_rgName="myRgName"

    az ad sp create-for-rbac --name "GitHubConnection" --role owner --scopes /subscriptions/$az_subId/resourceGroups/$az_rgName --sdk-auth
    ```

    > Important: The principal will need Owner permission on the target resource group scope as a minimum.

## Set up 

The following uses the provided GitHub workflows to build and deploy the sample. See Annex below for manual set up instructions.

### Deploy using GitHub

![GitHub workflow](https://github.com/axgonz/azure-nextflow/actions/workflows/cicd.yml/badge.svg?branch=main)

1. Create a new `AZURE_CREDENTIALS` GitHub Secret using the Service Principal created earlier; details in [Azure Docs](https://docs.microsoft.com/en-us/azure/developer/github/connect-from-azure?tabs=azure-cli%2Cwindows#create-a-service-principal-and-add-it-as-a-github-secret).

1. Run the workflow called `GitHub Workflows`.

    <img src="./docs/GitHubWorkflow.png" width="300" alt="Running the GitHub Workflow">

### Validate deployment

To validate set up was successful, trigger the Azure Function using its webhook.

``` bash
az_funcAppName="myFuncAppName"

curl --get "https://$az_funcAppName.azurewebsites.net/api/nxfutil"

# returns
Hello, World!
```

## Usage

When the Function App is triggered it will create a new nxfutil Container Instance. See Annex for nxfutil details.

The http trigger requires a json payload to provide the nextflow job with it's required config, pipeline and parameters files.

> NOTE: providing empty values will trigger a default deployment which uses the nextflow files in this repository.

``` json
{
    "config_uri": "",
    "pipeline_uri": "",
    "parameters_uri": ""
}
```

The http trigger currently accepts 1 (optional) argument.
- A boolean `whatif=true` argument can be provided to perform a mock deployment.

``` bash
az_funcAppName="nxfutil-py"

curl -X POST "https://$az_funcAppName.azurewebsites.net/api/nxfutil?whatif=true" -H 'Content-Type: application/json' -d '{"config_uri":"", "pipeline_uri":"", "parameters_uri":""}'
```

## Annex

### [Azure infrastructure](./docs/AzureInfrastructure.md)

### [nxfutil](./docs/nxfutil.md)

### [Azure functions](./azure/functions/python/README.md)

### [Data upload](./docs/DataUpload.md)

### [DevOps pipelines (depreciated)](./docs/DevOpsPipelines.md)

### [Manual set up (depreciated)](./docs/ManualSetup.md)

