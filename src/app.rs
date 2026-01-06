use crate::handlers;
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router(state: AppState) -> Router {
    let api_v1 = Router::new()
        .route("/today", get(handlers::get_today))
        .route("/stats", get(handlers::get_stats))
        .route("/click", post(handlers::click));

    Router::new()
        .route("/", get(handlers::index))
        .route("/click/add", post(handlers::click_add))
        .route("/click/sub", post(handlers::click_sub))
        .nest("/api/v1", api_v1.clone())
        .route("/api/today", get(handlers::get_today))
        .route("/api/stats", get(handlers::get_stats))
        .route("/api/click", post(handlers::click))
        .with_state(state)
}
