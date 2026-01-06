use crate::errors::AppError;
use crate::models::AppData;
use std::{env, path::Path, path::PathBuf};
use tokio::fs;
use tracing::error;

pub fn resolve_data_path() -> Result<PathBuf, std::io::Error> {
    if let Ok(path) = env::var("APP_DATA_PATH") {
        return Ok(PathBuf::from(path));
    }

    Ok(PathBuf::from("data/state.json"))
}

pub async fn load_data(path: &Path) -> AppData {
    match fs::read(path).await {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(data) => data,
            Err(err) => {
                error!("failed to parse data file: {err}");
                AppData::default()
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => AppData::default(),
        Err(err) => {
            error!("failed to read data file: {err}");
            AppData::default()
        }
    }
}

pub async fn persist_data(path: &Path, data: &AppData) -> Result<(), AppError> {
    let payload = serde_json::to_vec_pretty(data).map_err(AppError::internal)?;
    fs::write(path, payload).await.map_err(AppError::internal)?;
    Ok(())
}
