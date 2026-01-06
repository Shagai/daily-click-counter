use crate::models::AppData;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub data_path: PathBuf,
    pub data: Arc<Mutex<AppData>>,
}

impl AppState {
    pub fn new(data_path: PathBuf, data: AppData) -> Self {
        Self {
            data_path,
            data: Arc::new(Mutex::new(data)),
        }
    }
}
