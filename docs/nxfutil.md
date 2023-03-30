# nxfutil

nxfutil extends the Nextflow program to provide additional integration with Azure. The utility executes in the context of a container to meet the dependencies of Nextflow.

## Usage

Once deployed the Container Instance will start and execute the default command of `./nxfutil` or that provided when the Container Instance was deployed.

Once called, nxfutil will download the default or provided Nextflow files and parse the "nextflow.config" to create a list of secrets it will need to retrieve from Key Vault. At this time, it will also expand and replace any `exParams` parameters with their values (also retrieved from Key Vault).

Once the config file has been parsed nxfutil will show the resultant Nextflow config by running `nextflow config` and will finally offload to nextflow by running `nextflow run` specifying the pipeline and parameters files.

After the `nextflow` command completes successfully the Container Instance will stop.

## Lifecycle

A new nextflow Container Instance is needed for each different Nextflow pipeline, parameters and configuration combination.

Once a Container Instance executes and terminates it can be safely deleted unless the same job is to be dispatched again (using the same Nextflow pipeline, parameters and configuration files).
