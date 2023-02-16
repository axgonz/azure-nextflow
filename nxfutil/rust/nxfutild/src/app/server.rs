#[derive(Clone)]
pub struct AppServer {
    variables: AppVariables,
    az_storage_queues: AppAzStorageQueues
}

impl AppServer {
    fn new() -> Self {
        let app_vars = AppVariables::new();
        let az_identity = AppAzIdentity::new();
        let az_storage_queues = AppAzStorageQueues::new(az_identity.clone().credential, &app_vars);
        AppServer {
            variables: app_vars,
            az_storage_queues: az_storage_queues
        }
    }
    async fn init_services(app_server: &AppServer) {
        dbg!(&app_server.variables);
        let response = app_server.az_storage_queues.queue_client.create().await;   
        dbg!(response.unwrap());
    }
    async fn send_message_to_queue(Json(req_payload): Json<Value>, app_server: AppServer) {
        let _response = app_server.az_storage_queues.queue_client
            .put_message(req_payload.to_string()).await;
    }
    fn log(line: String) {
        println!("[nxfutilstatus] {}", line);
    }
}