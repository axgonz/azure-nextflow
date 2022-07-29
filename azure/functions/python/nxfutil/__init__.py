import logging
import azure.functions as func

import os
import time
import random
import string

from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient
from azure.mgmt.resource import ResourceManagementClient
from azure.mgmt.containerinstance import ContainerInstanceManagementClient
from azure.mgmt.containerinstance.models import (
    ContainerGroup,
    Container,
    ContainerGroupIdentity,
    ContainerGroupRestartPolicy,
    EnvironmentVariable,
    ResourceRequests,
    ResourceRequirements,
    OperatingSystemTypes,
    ImageRegistryCredential)

def main(req: func.HttpRequest) -> func.HttpResponse:
    container_name = 'nextflow-' + ''.join(random.choice(string.digits) for _ in range(6))
    log_prefix = f"NXFUTIL: [{container_name}]"
    
    logging.info(f"{log_prefix} Python HTTP trigger function processed a request.")

    NXFUTIL_AZ_SUB_ID = os.environ["NXFUTIL_AZ_SUB_ID"]
    NXFUTIL_AZ_RG_NAME = os.environ["NXFUTIL_AZ_RG_NAME"]
    NXFUTIL_AZ_KV_NAME = os.environ["NXFUTIL_AZ_KV_NAME"]
    NXFUTIL_AZ_CR_NAME = os.environ["NXFUTIL_AZ_CR_NAME"]
    NXFUTIL_AZ_MSI_ID = os.environ["NXFUTIL_AZ_MSI_ID"]

    logging.info(f"{log_prefix} App settings variables loaded:")
    logging.info(f"{log_prefix} NXFUTIL_AZ_SUB_ID = {NXFUTIL_AZ_SUB_ID}.")
    logging.info(f"{log_prefix} NXFUTIL_AZ_RG_NAME = {NXFUTIL_AZ_RG_NAME}.")
    logging.info(f"{log_prefix} NXFUTIL_AZ_KV_NAME = {NXFUTIL_AZ_KV_NAME}.")
    logging.info(f"{log_prefix} NXFUTIL_AZ_CR_NAME = {NXFUTIL_AZ_CR_NAME}.")
    logging.info(f"{log_prefix} NXFUTIL_AZ_MSI_ID = {NXFUTIL_AZ_MSI_ID}.")

    nxf_config = req.params.get('config')
    nxf_pipeline = req.params.get('pipeline')
    nxf_parameters = req.params.get('parameters')

    nxfutil_cmd = "nxfutil"

    if nxf_config:
        nxfutil_cmd = f"{nxfutil_cmd} -c {nxf_config}"
        
    if nxf_pipeline:
        nxfutil_cmd = f"{nxfutil_cmd} -p {nxf_pipeline}"

    if nxf_parameters:
        nxfutil_cmd = f"{nxfutil_cmd} -a {nxf_parameters}"

    bash_cmd = [f"/bin/bash", "-c", f"cd /.nextflow && ./{nxfutil_cmd}"]

    logging.info(f"{log_prefix} Generated nxfutil command: {bash_cmd}.")

    # Credential (N.B. When developing locally use 'az login' in any terminal to store a token)
    credential = DefaultAzureCredential()
    
    logging.info(f"{log_prefix} Credential obtained.")

    # Clients
    secret_client = SecretClient(
        vault_url = f"https://{NXFUTIL_AZ_KV_NAME}.vault.azure.net", 
        credential = credential)

    logging.info(f"{log_prefix} SecretClient Python SDK client initialised.")

    resource_client = ResourceManagementClient(
        credential = credential,
        subscription_id = NXFUTIL_AZ_SUB_ID
    )

    logging.info(f"{log_prefix} ResourceManagementClient Python SDK client initialised.")

    containerinstance_client = ContainerInstanceManagementClient(
        credential = credential,
        subscription_id = NXFUTIL_AZ_SUB_ID
    )

    logging.info(f"{log_prefix} ContainerInstanceManagementClient Python SDK client initialised.")

    # Get the resource group
    logging.info(f"{log_prefix} Retrieving Resource Group...")

    resource_group = resource_client.resource_groups.get(NXFUTIL_AZ_RG_NAME)

    logging.info(f"{log_prefix} Resource Group '{resource_group.name}' exits.")

    # Define the EnvironmentVariable
    AZ_KEY_VAULT_NAME = EnvironmentVariable(
        name = 'AZ_KEY_VAULT_NAME', 
        value = NXFUTIL_AZ_KV_NAME)

    AZURE_CLIENT_ID = EnvironmentVariable(
        name = 'AZURE_CLIENT_ID', 
        value = NXFUTIL_AZ_MSI_ID)

    logging.info(f"{log_prefix} ACI EnvironmentVariable defined.")

    # Define the ResourceRequests
    container_resource_requests = ResourceRequests(
        memory_in_gb = 1, 
        cpu = 1.0)

    logging.info(f"{log_prefix} ACI ResourceRequests defined.")
    
    # Define the ResourceRequirements
    container_resource_requirements = ResourceRequirements(
        requests = container_resource_requests)

    logging.info(f"{log_prefix} ACI ResourceRequirements requirements defined.")

    # Define the ImageRegistryCredential
    logging.info(f"{log_prefix} Retrieving Key Vault secrets...")

    acr_credential = ImageRegistryCredential(
        server = secret_client. get_secret('azure-registry-server'),
        username = secret_client.get_secret('azure-registry-username'),
        password = secret_client.get_secret('azure-registry-password')
    )

    logging.info(f"{log_prefix} ACI ImageRegistryCredential defined.")

    # Define the Container
    container = Container(
        name = container_name,
        image = f"{NXFUTIL_AZ_CR_NAME}.azurecr.io/default/nextflow:latest",
        resources = container_resource_requirements,
        command = bash_cmd,
        environment_variables = [
            AZ_KEY_VAULT_NAME, AZURE_CLIENT_ID])

    logging.info(f"{log_prefix} ACI Container defined.")

    # Define the ContainerGroupIdentity
    user_assigned_identity_key = f"/subscriptions/{NXFUTIL_AZ_SUB_ID}/resourcegroups/{NXFUTIL_AZ_RG_NAME}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/nextflowmid"
    user_assigned_identity_value = f"{{}}"

    mid_string = f"{{'{user_assigned_identity_key}':{user_assigned_identity_value}}}"
    mid_dict = eval(mid_string)

    mid = ContainerGroupIdentity(
        type = 'UserAssigned',
        user_assigned_identities = mid_dict)

    logging.info(f"{log_prefix} ACI ContainerGroupIdentity defined.")

    # Define the ContainerGroup
    group = ContainerGroup(
        location = resource_group.location,
        containers = [container],
        os_type = OperatingSystemTypes.linux,
        restart_policy = ContainerGroupRestartPolicy.never,
        identity = mid,
        image_registry_credentials = [acr_credential])

    logging.info(f"{log_prefix} ACI ContainerGroup defined.")

    # Create the instance
    logging.info(f"{log_prefix} Creating container instance...")
    
    result = containerinstance_client.container_groups.begin_create_or_update(
        resource_group_name = resource_group.name,
        container_group = group,
        container_group_name = container_name)
    
    # Wait for the container create operation to complete. The operation is
    # "done" when the container group provisioning state is one of:
    # Succeeded, Canceled, Failed
    while result.done() is False:
        time.sleep(1)

    # Get the provisioning state of the container group.
    container_group = containerinstance_client.container_groups.get(
        resource_group_name = resource_group.name,
        container_group_name = container_name)

    if str(container_group.provisioning_state).lower() == 'succeeded':           
        logging.info(f"{log_prefix} Successfully created container instance.")
        return func.HttpResponse(
            f"Successfully created nxfutil container instance {container_name}.",
            status_code=200)
    else:
        logging.info(f"{log_prefix} Failed to create container instance.")
        return func.HttpResponse(
            f"Failed to create nxfutil container instance {container_name}.",
            status_code=200)
