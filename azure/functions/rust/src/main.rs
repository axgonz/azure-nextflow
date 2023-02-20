use axum::{
    Router, 
    routing::{get, post}, 
    Server, 
    Json, 
    response::IntoResponse, 
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use core::panic;
use std::{env, fmt, thread, time, time::SystemTime, net::SocketAddr, collections::HashMap, sync::Arc, str::FromStr};
use strum_macros::EnumString;
use chrono::offset::Utc;
use chrono::DateTime;
use uuid::Uuid;

use azure_identity::{DefaultAzureCredential, DefaultAzureCredentialEnum, AzureCliCredential, ImdsManagedIdentityCredential};
use azure_security_keyvault::SecretClient;
use azure_mgmt_resources;
use azure_mgmt_containerinstance;

// Boiler-plate main fn: http server for Azure Functions
#[tokio::main]
async fn main() {
    // Config name of the function as defined in project file structure
    let name: &str = "nxfutil";

    // Grab the port from the azure functions runtime; or use port 3000.
    let port_key: &str = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    // Define the function address, this will be a binary on azure functions listen on localhost.
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Define the function path
    let path_string: String = format!("/api/{}", name);
    let path: &str = path_string.as_str();

    // Define our service with routes, any shared state and/or middleware (aka. exceptions), etc.
    let app = Router::new()
        .route(path, get(get_function))    
        .route(path, post(post_function));
        // Provide the state for the router
        // .with_state(state);
    
    // Start listening and panic if anything doesn't work.
    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

// Boiler-plate GET fn:
async fn get_function() -> &'static str {
    println!("Testing println!");
    dbg!("Testing dgb!");
    return "Hello, World!";
}

// Boiler-plate-ish POST fn:
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestPayload {
    pub config_uri: String,
    pub pipeline_uri: String,
    pub parameters_uri: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponsePayload {
    pub sub_id: String,
    pub rg_name: String,
    pub ci_name: String,
    pub ci_cmd: String,
    pub provisioning_state: String
}

fn log(line: String) {
    println!("[nxfutil] {}", line);
}

async fn post_function(
    axum::extract::Query(url_params):
    axum::extract::Query<HashMap<String, String>>,
    Json(req_payload): Json<RequestPayload>
) -> impl IntoResponse {
    log(format!("{:#?}", &req_payload));

    log("Checking url arguments for `whatif`".to_string());
    let what_if: bool = match url_params.get("whatif") {
        Some(key_value) => {
            log(format!("Found 'whatif' url param {:#?}", key_value));
            if key_value.to_lowercase() == "true" {
                true
            }
            else {
                false
            }
        },
        None => {
            false
        }
    };

    // Generate nxfutil command
    log("Generating nxfutil command from inputs".to_string());
    let nxfutil_cmd = generate_nxfutil_cmd(req_payload, url_params);

    // Request an access token
    log("Requesting access token".to_string());
    let credential = azure_login();

    // Read in environment variables
    log("Reading environment variables".to_string());
    let app_variables = AppVariables::new();
    log(format!("{:#?}", &app_variables));

    // Get key vault secrets
    log("Getting secrets from key vault".to_string());
    let app_secrets = get_app_secrets(credential.clone(), app_variables.clone()).await;

    // Deploy container instance
    log("Deploying nextflow container instance".to_string());
    let deployment = deploy_nxfutil_ci(credential, app_variables.clone(), app_secrets, nxfutil_cmd.clone(), what_if).await;

    // Generate ResponsePayload
    log("Generating ResponsePayload".to_string());
    let res_payload = ResponsePayload { 
        sub_id: app_variables.sub_id,
        rg_name: app_variables.rg_name,
        ci_name: deployment.0,
        ci_cmd: nxfutil_cmd,
        provisioning_state: deployment.1
    };
    log(format!("{:#?}", &res_payload));

    return (StatusCode::OK, Json(res_payload))
}

// App fn:
fn generate_nxfutil_cmd(req_payload: RequestPayload, url_params: HashMap<String, String>) -> String {
    /* nxfutil options for specifying 'config', 'pipeline' and 'parameters' files can
        be provided as either a json payload or as url arguments. Payload will take 
        precedence and url arguments will be depreciated.
    */
    log("Checking url arguments for `config_uri`".to_string());
    let mut config_uri: String = match url_params.get("config_uri") {
        Some(key_value) => {
            log(format!("Found 'config_uri' url param {:#?}", key_value));
            key_value.to_string()
        },
        None => {
            "".to_string()
        }
    };
    log("Checking RequestPayload for `config_uri`".to_string());
    if !req_payload.config_uri.is_empty() {
        log(format!("Found 'config_uri' in RequestPayload {:#?}", req_payload.config_uri));
        config_uri = req_payload.config_uri;
    }
    
    log("Checking url arguments for `pipeline_uri`".to_string());
    let mut pipeline_uri: String = match url_params.get("pipeline_uri") {
        Some(key_value) => {
            log(format!("Found 'pipeline_uri' url param {:#?}", key_value));
            key_value.to_string()
        },
        None => {
            "".to_string()
        }
    };
    log("Checking RequestPayload for `pipeline_uri`".to_string());
    if !req_payload.pipeline_uri.is_empty() {
        log(format!("Found 'pipeline_uri' in RequestPayload {:#?}", req_payload.pipeline_uri));
        pipeline_uri = req_payload.pipeline_uri;
    }

    log("Checking url arguments for `parameters_uri`".to_string());
    let mut parameters_uri: String = match url_params.get("parameters_uri") {
        Some(key_value) => {
            log(format!("Found 'parameters_uri' url param {}", key_value));
            key_value.to_string()
        },
        None => {
            "".to_string()
        }
    };
    log("Checking RequestPayload for `parameters_uri`".to_string());
    if !req_payload.parameters_uri.is_empty() {
        log(format!("Found 'parameters_uri' in RequestPayload {:#?}", req_payload.parameters_uri));
        parameters_uri = req_payload.parameters_uri;
    }

    let mut nxfutil_cmd: String = "nxfutil".to_string();
    if !config_uri.is_empty() {
        nxfutil_cmd = format!("{} -c {}", nxfutil_cmd, config_uri);
    }
    if !pipeline_uri.is_empty() {
        nxfutil_cmd = format!("{} -p {}", nxfutil_cmd, pipeline_uri);
    }
    if !parameters_uri.is_empty() {
        nxfutil_cmd = format!("{} -a {}", nxfutil_cmd, parameters_uri);
    }
    log(format!("Generated nextflow cmd is {:#?}", &nxfutil_cmd));
    
    return nxfutil_cmd
}

fn azure_login() -> Arc<DefaultAzureCredential> {
    /* Build Azure credential
        If using a User Assigned Managed Identity you must set the `AZURE_CLIENT_ID`
        environment variable to give `DefaultAzureCredential::default()` the right context. 
        
        This (`AZURE_CLIENT_ID`) can be omitted if using a System Assigned Managed Identity 
        and when developing locally. 

        When developing locally make sure to run `az login` somewhere on your local PC to cache
        your own credential for the builder to use; do not provide `AZURE_CLIENT_ID`.
    */  
    let credential_sources = match env::var("AZURE_CLIENT_ID") {
        Ok(azure_client_id) => {
            log("AZURE_CLIENT_ID is set, will try to use User Assigned Managed Identity".to_string());
            vec![
                DefaultAzureCredentialEnum::ManagedIdentity(
                    ImdsManagedIdentityCredential::default()
                        .with_client_id(azure_client_id)
                ),
                DefaultAzureCredentialEnum::AzureCli(
                    AzureCliCredential::new()
                )
            ]
        },
        Err(_) => {
            log("AZURE_CLIENT_ID is unset, will try to use System Assigned Managed Identity".to_string());
            vec![
                DefaultAzureCredentialEnum::ManagedIdentity(
                    ImdsManagedIdentityCredential::default()
                ),
                DefaultAzureCredentialEnum::AzureCli(
                    AzureCliCredential::new()
                )
            ]
        }
    };    

    return Arc::new(DefaultAzureCredential::with_sources(credential_sources));
}

//# AppVariables
#[derive(Debug, Clone)]
pub struct AppVariables {
    pub sub_id: String,
    pub rg_name: String,
    pub kv_name: String,
    pub cr_name: String,
    pub msi_name: String,
    pub msi_client_id: String
}

impl Default for AppVariables {
    fn default() -> Self {
        Self {
            sub_id: env::var("NXFUTIL_AZ_SUB_ID").unwrap(),
            rg_name: env::var("NXFUTIL_AZ_RG_NAME").unwrap(),
            kv_name: env::var("NXFUTIL_AZ_KV_NAME").unwrap(),
            cr_name: env::var("NXFUTIL_AZ_CR_NAME").unwrap(),
            msi_name: env::var("NXFUTIL_AZ_MSI_NAME").unwrap(),
            msi_client_id: env::var("NXFUTIL_AZ_MSI_ID").unwrap() 
        }
    }
}

impl AppVariables {
    pub fn new() -> Self {
        Self::default()
    }
}
//#end AppVariables

//# AppSecrets
#[derive(Debug, Clone)]
pub struct AppSecrets {
    pub cr_server: String,
    pub cr_username: String,
    pub cr_password: String
}

impl Default for AppSecrets {
    fn default() -> Self {
        Self {
            cr_server: "".to_string(),
            cr_username: "".to_string(),
            cr_password: "".to_string(),
        }
    }
}

impl AppSecrets {
    pub fn new() -> Self {
        Self::default()
    }
}

async fn get_kv_secret(secret_name: &str, client: &SecretClient) -> String {
    log(format!("Request key vault secret {:#?}", secret_name));
    let secret = match client.get(secret_name).await {
        Ok(secret) => {
            log(format!("Received key vault secret {:#?}", secret_name));
            secret
        },
        Err(error) => {
            log(format!("Error retrieving key vault secret {:#?}", secret_name));
            panic!("{}", error);
        }
    };
    return secret.value;
}

async fn get_app_secrets(credential: Arc<DefaultAzureCredential>, app_variables: AppVariables) -> AppSecrets {
    log(format!("Establishing azure_security_keyvault::SecretClient for {:#?}", app_variables.kv_name));
    let kv_url = format!("https://{}.vault.azure.net", app_variables.kv_name);
    let client = match SecretClient::new(
        &kv_url, 
        credential
    ) {
        Ok(client) => {
            log(format!("Established azure_security_keyvault::SecretClient for {:#?}", app_variables.kv_name));
            client
        },
        Err(error) => {
            log(format!("Failed to establish azure_security_keyvault::SecretClient for {:#?}", app_variables.kv_name));
            panic!("{}", error)
        }
    };

    let mut app_secrets = AppSecrets::new();
    app_secrets.cr_server = get_kv_secret("azure-registry-server", &client).await;
    app_secrets.cr_username = get_kv_secret("azure-registry-username", &client).await;
    app_secrets.cr_password = get_kv_secret("azure-registry-password", &client).await;
    return app_secrets;
}
//#end AppSecrets

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

async fn deploy_nxfutil_ci (
    credential: Arc<DefaultAzureCredential>, 
    app_variables: AppVariables,
    app_secrets: AppSecrets,
    nxfutil_cmd: String,
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
    log("Generating unique container instance resource name".to_string());
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
    log(format!("Generated unique container instance resource name {:#?}", ci_name));

    // 2
    log("Retrieving resource group to determine the deployment location".to_string());
    let azure_mgmt_resources = azure_mgmt_resources::Client::builder(credential.clone()).build();
    let rg_client = azure_mgmt_resources.resource_groups_client();  
    let rg = match rg_client.get(
        app_variables.rg_name.clone(), 
        app_variables.sub_id.clone()
    ).into_future().await {
        Ok(rg) => {
            log(format!("Resource group location is {:#?}", rg.location));
            rg
        },
        Err(error) => {
            log(format!("Error retrieving resource group {:#?}", app_variables.rg_name));
            panic!("{}", error)
        }
    };
    
    // 3
    log("Defining the container instance user assigned identity".to_string());
    let ci_user_assigned_identities_json = serde_json::from_str(format!(
        "{{\"/subscriptions/{}/resourcegroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}\":{{}}}}", 
        app_variables.sub_id,
        app_variables.rg_name, 
        app_variables.msi_name
    ).as_str());
    log(format!("Defined container instance user assigned identity as {:#?}", ci_user_assigned_identities_json));

    log("Defining the container instance".to_string());
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
                            image: format!("{}.azurecr.io/default/nextflow:latest", app_variables.cr_name),
                            command: vec![
                                "/bin/bash".to_string(), 
                                "-c".to_string(), 
                                format!("cd /.nextflow && ./{}", nxfutil_cmd)
                            ], 
                            environment_variables: vec![
                                azure_mgmt_containerinstance::models::EnvironmentVariable {
                                    name: "AZURE_CLIENT_ID".to_string(),
                                    value: Some(app_variables.msi_client_id),
                                    secure_value: None
                                },
                                azure_mgmt_containerinstance::models::EnvironmentVariable {
                                    name: "AZURE_KEYVAULT_NAME".to_string(),
                                    value: Some(app_variables.kv_name),
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
                    server: app_secrets.cr_server,
                    username: Some(app_secrets.cr_username),
                    password: Some(app_secrets.cr_password),
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
    log("Defined container instance".to_string());

    // 4
    log("Establishing azure_mgmt_containerinstance::Client".to_string());
    let azure_mgmt_containerinstance = azure_mgmt_containerinstance::Client::builder(credential).build();
    let ci_group_client = azure_mgmt_containerinstance.container_groups_client();

    if what_if {
        let deployment_result = ProvisioningState::WhatIf.to_string();
        return (ci_name, deployment_result);
    }

    log("Submitting container instance deployment".to_string());
    let mut deployment_result = match ci_group_client.create_or_update(
        app_variables.sub_id.clone(), 
        app_variables.rg_name.clone(), 
        ci_name.clone(),
        ci_group
    ).into_future().await {
        Ok(ci_group_result) => {
            log("Deployment was submitted without errors".to_string());
            ci_group_result.container_group_properties.properties.provisioning_state.unwrap()
        },
        Err(error) => {
            log(format!("Failed to submit deployment. Error {:#?}", error));
            ProvisioningState::Failed.to_string()
        }
    };

    log(format!("Deployment result {:#?}", deployment_result));

    let mut provisioning = match ProvisioningState::from_str(&deployment_result) {
        Ok(ProvisioningState::Succeeded) => false,
        Ok(ProvisioningState::Failed) => false,
        Ok(ProvisioningState::Canceled) => false,
        _ => true
    };  

    // 5
    let delay_seconds = 15;
    let delay_duration = time::Duration::from_secs(delay_seconds);
    
    while provisioning {
        log(format!("Waiting for {:#?} seconds...", delay_seconds));
        thread::sleep(delay_duration);

        deployment_result = match ci_group_client.get(
            app_variables.sub_id.clone(), 
            app_variables.rg_name.clone(), 
            ci_name.clone()
        ).into_future().await {
            Ok(ci_group_result) => ci_group_result.container_group_properties.properties.provisioning_state.unwrap(),
            Err(_) => ProvisioningState::Failed.to_string()
        };

        log(format!("Deployment result {:#?}", deployment_result));

        provisioning = match ProvisioningState::from_str(&deployment_result) {
            Ok(ProvisioningState::Succeeded) => false,
            Ok(ProvisioningState::Failed) => false,
            Ok(ProvisioningState::Canceled) => false,
            _ => true
        };
    };

    return (ci_name, deployment_result);
}