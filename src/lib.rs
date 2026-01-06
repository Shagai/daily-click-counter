pub mod app;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod stats;
pub mod storage;
pub mod ui;
pub mod state;

pub use app::router;
pub use state::AppState;
pub use storage::{load_data, resolve_data_path};
