use azure_storage:: {
    StorageCredentials
};
use azure_storage_queues:: {
    QueueServiceClient,
    QueueClient
};

#[derive(Clone)]
pub struct AppAzStorageQueues {
    pub queue_client: QueueClient
}

impl AppAzStorageQueues { 
    fn new(credential: Arc<DefaultAzureCredential>, app_vars: &AppVariables) -> Self {
        let storage_credentials = StorageCredentials::TokenCredential(credential);
        let queue_service = QueueServiceClient::new(&app_vars.st_name, storage_credentials);
        AppAzStorageQueues {
            queue_client: queue_service.queue_client(&app_vars.func_name)
        }
    }
}