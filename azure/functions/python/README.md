# Azure Function App

## Setup

When developing locally:

- Ensure the development account has been granted "secret list" and "secret read" permissions within Key Vault.
- Ensure to cache an access token locally (for DefaultAzureCredential to pick up), do this by running `az login` in any terminal on the development machine.
- Create a `local.settings.json` file in the same directory as this README and update variables that start with `NXFUTIL_`. 

    ``` json
    {
        "IsEncrypted": false,
        "Values": {
            "AzureWebJobsStorage": "",
            "FUNCTIONS_WORKER_RUNTIME": "python",
            "NXFUTIL_AZ_SUB_ID": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
            "NXFUTIL_AZ_RG_NAME": "myRgName",
            "NXFUTIL_AZ_KV_NAME": "myKvName",
            "NXFUTIL_AZ_CR_NAME": "myCrName",
            "NXFUTIL_AZ_MID_CLIENT_ID": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
        }
    }
    ```