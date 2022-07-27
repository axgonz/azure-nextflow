# Azure Nextflow

This repository contains sample only code to demonstrate how secrets can be marshaled from Key Vault into a Container Instance running Nextflow for the purpose of dispatching Nextflow pipelines on Azure Batch.

## Overview

Deploying the Azure resources supporting this sample is left to you.

It is assumed that this infrastructure would be deployed through a DevOps pipeline or GitHub workflow. 

This sample will be eventually expanded to include Bicep templates for capturing the state of required Azure resources and pipeline/workflow files for the deployment.

## Set up

1. Deploy Azure resources.

    ``` bash
    az_location="australiaeast"
    az_subId="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

    az deployment sub create --name "batch-$az_location" --location $az_location --subscription $az_subId --template-file ./azure/templates/main.bicep
    ```

1. Build Docker images.

    ``` bash
    cd ./docker/images/
    chmod +x build.sh
    ./build.sh
    ```

1. Push images to the Container Registry deployed in step #1.

    ``` bash
    # Update the 'ACR' variable before running
    nano ./docker/images/login.sh
    chmod +x ./docker/images/login.sh
    ./docker/images/login.sh

    # Update the 'ACR' variable before running
    nano ./docker/images/push.sh
    chmod +x ./docker/images/push.sh
    ./docker/images/push.sh
    ```

1. Validate deployment by running the default "Hello World" Nextflow pipeline provided with this sample.

    ``` bash
    az_subId="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
    az_rgName="myRgName"
    az_kvName="myKvName"
    az_crName="myCrName"
    az_midClientId="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

    az container create -g $az_rgName \
        --name nextflow1 \
        --image "algonz/nextflow:latest" \
        --cpu 1 \
        --memory 1 \
        --restart-policy Never \
        --environment-variables AZ_KEY_VAULT_NAME=$az_kvName AZURE_CLIENT_ID=$az_midClientId \
        --assign-identity "/subscriptions/$az_subId/resourcegroups/$az_rgName/providers/Microsoft.ManagedIdentity/userAssignedIdentities/nextflowmid"
    ```

## Usage

Once deployed the Container Instance will start and execute the default command of `./nxfutil` or that provided when the Container Instance was deployed.

Once called, nxfutil will download the default or provided Nextflow files and parse the "nextflow.config" to create a list of secrets it will need to retrieve from Key Vault. At this time, it will also expand and replace any `exParams` parameters with their values (also retrieved from Key Vault).

Once the config file has been parsed nxfutil will show the resultant Nextflow config by running `nextflow config` and will finally offload to nextflow by running `nextflow run` specifying the pipeline and parameters files.

After the `nextflow` command completes successfully the Container Instance will stop. 

It is intended that the next iteration of nxfutil will integrate with Nextflow Tower so that launched jobs can be monitored remotely. It is also possible that the utility will be re-written using nodejs, dotNet and/or Rust.

## Lifecycle

A new nextflow Container Instance is needed for each different Nextflow pipeline, parameters and configuration combination.

Once a Container Instance executes and terminates it can be safely deleted unless the same job is to be dispatched again (using the same Nextflow pipeline, parameters and configuration files).

The quickest way to dispatched different Nextflow jobs is to use the Azure Cli to create a new nextflow Container Instance.

``` bash
az_subId="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
az_rgName="myRgName"
az_kvName="myKvName"
az_crName="myCrName"
az_midClientId="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

nxf_configUri="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/nextflow.config"
nxf_pipelineUri="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/pipeline.nf"
nxf_parametersUri="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/parameters.json"

az container create -g $az_rgName \
    --name nextflow1 \
    --image "$az_crName.azurecr.io/default/nextflow:latest" \
    --cpu 1 \
    --memory 1 \
    --restart-policy Never \
    --environment-variables AZ_KEY_VAULT_NAME=$az_kvName AZURE_CLIENT_ID=$az_midClientId \
    --assign-identity "/subscriptions/$az_subId/resourcegroups/$az_rgName/providers/Microsoft.ManagedIdentity/userAssignedIdentities/nextflowmid" \
    --command-line "/bin/bash -c 'cd /.nextflow && ./nxfutil -c $nxf_configUri -p $nxf_pipelineUri -a $nxf_parametersUri'"
```

> N.B. The point-in-time Docker build of the nextflow image used in this sample is available on Docker Hub. To use it replace `--image ...` in the command above with `--image algonz/nextflow:latest`

## Annexes

### [Azure infrastructure](./doc/AzureInfrastructure.md)

### [Data upload](./doc/DataUpload.md)
