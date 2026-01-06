use std::{env, net::SocketAddr};
use tokio::fs;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let data_path = web_app::resolve_data_path()?;
    if let Some(parent) = data_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let data = web_app::load_data(&data_path).await;
    let state = web_app::AppState::new(data_path, data);

    let app = web_app::router(state);

    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
