use az_app_identity::*;

use crate::app::{
    variables::*,
};

use crate::services::{
    az_storage_queues::*,
};

pub use std::{
    sync::Arc,
    sync::Mutex
};

// AppState will eventually be wrapped in an Arc, 
//  mutex necessary to mutate safely across threads.
pub struct AppState {
    pub identity: Arc<DefaultAzureCredential>,
    pub variables: AppVariables,
    pub queue: AppAzStorageQueue,
}
