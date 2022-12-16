## Manual set up (depreciated)

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
