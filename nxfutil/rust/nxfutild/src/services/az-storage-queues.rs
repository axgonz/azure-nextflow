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
    fn new(credential: Arc<DefaultAzureCredential>, variables: &AppVariables, secrets: &AppSecrets) -> Self {
        let storage_credentials = StorageCredentials::TokenCredential(credential);
        let queue_service = QueueServiceClient::new(&secrets.st_name, storage_credentials);
        AppAzStorageQueues {
            queue_client: queue_service.queue_client(&variables.fc_name)
        }
    }
}