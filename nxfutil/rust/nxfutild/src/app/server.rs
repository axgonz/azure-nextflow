#[derive(Clone)]
pub struct AppServer {
    variables: AppVariables,
    secrets: AppSecrets,
    az_identity: AppAzIdentity,
    az_storage_queues: AppAzStorageQueues
}

impl AppServer {
    fn new(variables: AppVariables, secrets: AppSecrets, az_identity: AppAzIdentity) -> Self {
        Self {
            az_storage_queues: AppAzStorageQueues::new(az_identity.credential.clone(), &variables, &secrets),
            az_identity: az_identity,
            variables: variables,
            secrets: secrets
        }
    }
    async fn init(server: &AppServer) {
        print!("[az-storage-queues] Creating queue if not exists {:#?}...", server.variables.fc_name);
        io::stdout().flush().unwrap();
        match server.az_storage_queues.queue_client.create().await {
            Ok(_) => {
                println!("Ok");
            },
            Err(error) => {
                println!("Err");
                panic!("{}", error)
            }
        }
    }
    async fn send_message_to_queue(Json(req_payload): Json<Value>, server: &AppServer) {
        print!("[az-storage-queues] Sending message...");
        io::stdout().flush().unwrap();
        match server.az_storage_queues.queue_client.put_message(req_payload.to_string()).await {
            Ok(_) => {
                println!("Ok");
            },
            Err(error) => {
                println!("Err");
                println!("{}", error)
            }        
        }
    }
}