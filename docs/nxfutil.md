# nxfutil

nxfutil extends the Nextflow program to provide additional integration with Azure. The utility executes in the context of a container to meet the dependencies of Nextflow.

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