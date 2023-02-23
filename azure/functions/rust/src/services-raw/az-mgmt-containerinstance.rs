use azure_mgmt_resources;
use azure_mgmt_containerinstance;

use std::{
    fmt, 
    thread, 
    time::SystemTime,
    time::Duration,
    str::FromStr
};

use strum_macros::{
    EnumString
};

use chrono::{
    offset::Utc,
    DateTime
};

use uuid::{
    Uuid
};

#[derive(EnumString)]
pub enum ProvisioningState {
    Succeeded = 0,
    Failed = 1,
    Canceled = 2,
    InProgress = 3,
    Deleting = 4,
    WhatIf = 256
}

impl fmt::Display for ProvisioningState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProvisioningState::Succeeded => write!(f, "Succeeded"),
            ProvisioningState::Failed => write!(f, "Failed"),
            ProvisioningState::Canceled => write!(f, "Canceled"),
            ProvisioningState::InProgress => write!(f, "InProgress"),
            ProvisioningState::Deleting => write!(f, "Deleting"),
            ProvisioningState::WhatIf => write!(f, "WhatIf"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppAzMgmtContainerInstance {}

impl AppAzMgmtContainerInstance { 
    async fn create_nxfutil_ci(
        credential: Arc<DefaultAzureCredential>, 
        variables: &AppVariables,
        secrets: &AppSecrets,
        nxfutil_cmd: &String,
        what_if: bool
    ) -> (String, String) {
        /* Steps to define and build the container instance
            1. Create a unique name
            2. Get the resource group to determine the deployment location
            3. Define the container instance
            4. POST the deployment to ARM
            5. Wait for the deployment to complete
        */
    
        // 1
        println!("[handler] Generating unique container instance resource name");
        let unique_id = Uuid::new_v4();
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        let datetime_string = format!("{}", datetime.format("%Y%m%d"));
        
        /* Azure naming limits: Microsoft.ContainerInstance
            Entity          | Scope          | Length | Valid Characters
            containerGroups | resource group | 1-63   | Lowercase letters, numbers, and hyphens.
                                                        Can't start or end with hyphen. 
                                                        Consecutive hyphens aren't allowed.
        */
        let ci_name = format!("nextflow-{}-{}", datetime_string, unique_id);
        println!("[handler] Generated unique container instance resource name {:#?}", ci_name);
    
        // 2
        println!("[handler] Retrieving resource group to determine the deployment location");
        let azure_mgmt_resources = azure_mgmt_resources::Client::builder(credential.clone()).build();
        let rg_client = azure_mgmt_resources.resource_groups_client();  
        let rg = match rg_client.get(
            variables.rg_name.clone(), 
            variables.sub_id.clone()
        ).await {
            Ok(rg) => {
                println!("[handler] Resource group location is {:#?}", rg.location);
                rg
            },
            Err(error) => {
                println!("[handler] Error retrieving resource group {:#?}", variables.rg_name);
                panic!("{}", error)
            }
        };
        
        // 3
        println!("[handler] Defining the container instance user assigned identity");
        let ci_user_assigned_identities_json = serde_json::from_str(format!(
            "{{\"/subscriptions/{}/resourcegroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}\":{{}}}}", 
            variables.sub_id,
            variables.rg_name, 
            variables.msi_name
        ).as_str());
        println!("[handler] Defined container instance user assigned identity as {:#?}", ci_user_assigned_identities_json);
    
        println!("[handler] Defining the container instance");
        let ci_group = azure_mgmt_containerinstance::models::ContainerGroup {
            container_group_properties: azure_mgmt_containerinstance::models::ContainerGroupProperties {
                identity: Some(azure_mgmt_containerinstance::models::ContainerGroupIdentity {
                    type_: Some(azure_mgmt_containerinstance::models::container_group_identity::Type::UserAssigned),
                    user_assigned_identities: Some(ci_user_assigned_identities_json.unwrap()),
                    principal_id: None,
                    tenant_id: None,
                }),
                properties: azure_mgmt_containerinstance::models::container_group_properties::Properties { 
                    provisioning_state: None, 
                    containers: vec![azure_mgmt_containerinstance::models::Container {
                        name: ci_name.clone(), 
                        properties: {
                            azure_mgmt_containerinstance::models::ContainerProperties { 
                                image: format!("{}.azurecr.io/default/nextflow:latest", variables.cr_name),
                                command: vec![
                                    "/bin/bash".to_string(), 
                                    "-c".to_string(), 
                                    format!("cd /.nextflow && ./{}", nxfutil_cmd)
                                ], 
                                environment_variables: vec![
                                    azure_mgmt_containerinstance::models::EnvironmentVariable {
                                        name: "AZURE_CLIENT_ID".to_string(),
                                        value: Some(variables.msi_client_id.clone()),
                                        secure_value: None
                                    },
                                    azure_mgmt_containerinstance::models::EnvironmentVariable {
                                        name: "AZURE_KEYVAULT_NAME".to_string(),
                                        value: Some(variables.kv_name.clone()),
                                        secure_value: None
                                    },
                                    azure_mgmt_containerinstance::models::EnvironmentVariable {
                                        name: "AZURE_FUNCAPP_NAME".to_string(),
                                        value: Some(variables.fn_name.clone()),
                                        secure_value: None
                                    },
                                    azure_mgmt_containerinstance::models::EnvironmentVariable {
                                        name: "NXFUTIL_DISPATCHER".to_string(),
                                        value: Some(ci_name.clone()),
                                        secure_value: None
                                    }
                                ], 
                                resources: azure_mgmt_containerinstance::models::ResourceRequirements {
                                    requests: azure_mgmt_containerinstance::models::ResourceRequests { 
                                        memory_in_gb: 1.0, 
                                        cpu: 1.0,
                                        gpu: None
                                    },
                                    limits: None
                                },
                                ports: vec![],
                                volume_mounts: vec![], 
                                instance_view: None,
                                liveness_probe: None,
                                readiness_probe: None
                            }
                        }
                    }], 
                    image_registry_credentials: vec![azure_mgmt_containerinstance::models::ImageRegistryCredential {
                        server: secrets.cr_server.clone(),
                        username: Some(secrets.cr_username.clone()),
                        password: Some(secrets.cr_password.clone()),
                        identity: None,
                        identity_url: None
                    }], 
                    os_type: azure_mgmt_containerinstance::models::container_group_properties::properties::OsType::Linux, 
                    restart_policy: Some(azure_mgmt_containerinstance::models::container_group_properties::properties::RestartPolicy::Never), 
                    ip_address: None, 
                    volumes: vec![], 
                    instance_view: None, 
                    diagnostics: None, 
                    subnet_ids: vec![],     
                    dns_config: None, 
                    sku: None, 
                    encryption_properties: None, 
                    init_containers: vec![], 
                    extensions: vec![]
                }
            },
            resource: azure_mgmt_containerinstance::models::Resource { 
                id: None, 
                name: None, 
                type_: None, 
                location: Some(rg.location), 
                tags: None, 
                zones: vec![] 
            }
        };
        println!("[handler] Defined container instance");
    
        // 4
        println!("[handler] Establishing azure_mgmt_containerinstance::Client");
        let azure_mgmt_containerinstance = azure_mgmt_containerinstance::Client::builder(credential).build();
        let ci_group_client = azure_mgmt_containerinstance.container_groups_client();
    
        if what_if {
            let deployment_result = ProvisioningState::WhatIf.to_string();
            return (ci_name, deployment_result);
        }
    
        println!("[handler] Submitting container instance deployment");
        let mut deployment_result = match ci_group_client.create_or_update(
            variables.sub_id.clone(), 
            variables.rg_name.clone(), 
            ci_name.clone(),
            ci_group
        ).await {
            Ok(ci_group_result) => {
                println!("[handler] Deployment was submitted without errors");
                ci_group_result.container_group_properties.properties.provisioning_state.unwrap()
            },
            Err(error) => {
                println!("[handler] Failed to submit deployment. Error {:#?}", error);
                ProvisioningState::Failed.to_string()
            }
        };
    
        println!("[handler] Deployment result {:#?}", deployment_result);
    
        let mut provisioning = match ProvisioningState::from_str(&deployment_result) {
            Ok(ProvisioningState::Succeeded) => false,
            Ok(ProvisioningState::Failed) => false,
            Ok(ProvisioningState::Canceled) => false,
            _ => true
        };  
    
        // 5
        let delay_seconds = 15;
        let delay_duration = Duration::from_secs(delay_seconds);
        
        while provisioning {
            println!("[handler] Waiting for {:#?} seconds...", delay_seconds);
            thread::sleep(delay_duration);
    
            deployment_result = match ci_group_client.get(
                variables.sub_id.clone(), 
                variables.rg_name.clone(), 
                ci_name.clone()
            ).await {
                Ok(ci_group_result) => ci_group_result.container_group_properties.properties.provisioning_state.unwrap(),
                Err(_) => ProvisioningState::Failed.to_string()
            };
    
            println!("[handler] Deployment result {:#?}", deployment_result);
    
            provisioning = match ProvisioningState::from_str(&deployment_result) {
                Ok(ProvisioningState::Succeeded) => false,
                Ok(ProvisioningState::Failed) => false,
                Ok(ProvisioningState::Canceled) => false,
                _ => true
            };
        };
    
        return (ci_name, deployment_result);
    }
    async fn delete_nxfutil_ci(
        credential: Arc<DefaultAzureCredential>, 
        variables: &AppVariables,
        ci_name: &String,
        what_if: bool
    ) -> (String, String) {
        /* Steps to define and build the container instance
            1. Get the resource group to determine the deployment location
            2. POST the deployment to ARM
            3. Wait for the deployment to complete
        */

        // 1
        println!("[handler] Retrieving resource group to determine the deployment location");
        let azure_mgmt_resources = azure_mgmt_resources::Client::builder(credential.clone()).build();
        let rg_client = azure_mgmt_resources.resource_groups_client();  
        let rg = match rg_client.get(
            variables.rg_name.clone(), 
            variables.sub_id.clone()
        ).await {
            Ok(rg) => {
                println!("[handler] Resource group location is {:#?}", rg.location);
                rg
            },
            Err(error) => {
                println!("[handler] Error retrieving resource group {:#?}", variables.rg_name);
                panic!("{}", error)
            }
        };

        // 2
        println!("[handler] Establishing azure_mgmt_containerinstance::Client");
        let azure_mgmt_containerinstance = azure_mgmt_containerinstance::Client::builder(credential).build();
        let ci_group_client = azure_mgmt_containerinstance.container_groups_client();
    
        if what_if {
            let deployment_result = ProvisioningState::WhatIf.to_string();
            return (ci_name.to_string(), deployment_result);
        }
    
        println!("[handler] Submitting container instance deployment");
        let mut deployment_result = match ci_group_client.delete(
            variables.sub_id.clone(), 
            variables.rg_name.clone(), 
            ci_name.clone()
        ).await {
            Ok(ci_group_result) => {
                println!("[handler] Deployment was submitted without errors");
                ci_group_result.container_group_properties.properties.provisioning_state.unwrap()
            },
            Err(error) => {
                println!("[handler] Failed to submit deployment. Error {:#?}", error);
                ProvisioningState::Failed.to_string()
            }
        };
    
        println!("[handler] Deployment result {:#?}", deployment_result);
    
        let mut provisioning = match ProvisioningState::from_str(&deployment_result) {
            Ok(ProvisioningState::Succeeded) => false,
            Ok(ProvisioningState::Failed) => false,
            Ok(ProvisioningState::Canceled) => false,
            _ => true
        };  
    
        // 3
        let delay_seconds = 15;
        let delay_duration = Duration::from_secs(delay_seconds);
        
        while provisioning {
            println!("[handler] Waiting for {:#?} seconds...", delay_seconds);
            thread::sleep(delay_duration);
    
            deployment_result = match ci_group_client.get(
                variables.sub_id.clone(), 
                variables.rg_name.clone(), 
                ci_name.clone()
            ).await {
                Ok(ci_group_result) => ci_group_result.container_group_properties.properties.provisioning_state.unwrap(),
                Err(_) => ProvisioningState::Failed.to_string()
            };
    
            println!("[handler] Deployment result {:#?}", deployment_result);
    
            provisioning = match ProvisioningState::from_str(&deployment_result) {
                Ok(ProvisioningState::Succeeded) => false,
                Ok(ProvisioningState::Failed) => false,
                Ok(ProvisioningState::Canceled) => false,
                _ => true
            };
        };
    
        return (ci_name.to_string(), deployment_result);
    }
}
