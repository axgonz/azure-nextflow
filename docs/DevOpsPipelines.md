### Deploy using DevOps 

![DevOps Pipeline](https://dev.azure.com/algonz/azure-nextflow/_apis/build/status/axgonz.azure-nextflow?branch=main)

1. Create a new `Azure Resource Manager - Service principal (manual)` Service Connection in DevOps using the Service Principal created earlier; details in [Azure Docs](https://docs.microsoft.com/en-us/azure/devops/pipelines/library/connect-to-azure?view=azure-devops#create-an-azure-resource-manager-service-connection-with-an-existing-service-principal).

1. Creating a new pipeline and link to the `./.devops/pipelines/cicd.yml` file that's on GitHub by selecting `GitHub (YAML)` and then `Existing Azure Pipelines YAML file`.

1. Run the newly created pipeline.

    <img src="./DevOpsPipeline.png" width="300" alt="Running the DevOps Pipeline">