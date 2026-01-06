use crate::errors::AppError;
use crate::models::{ClickRequest, DailyCountsResponse, DayCounts, StatsResponse};
use crate::state::AppState;
use crate::stats::build_stats;
use crate::storage::persist_data;
use crate::ui::render_index;
use axum::{
    extract::State,
    response::{Html, Redirect},
    Json,
};
use chrono::Local;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let date = today_string();
    let data = state.data.lock().await;
    let counts = data.days.get(&date).cloned().unwrap_or_default();
    Html(render_index(&date, &counts))
}

pub async fn get_today(State(state): State<AppState>) -> Result<Json<DailyCountsResponse>, AppError> {
    let date = today_string();
    let data = state.data.lock().await;
    let counts = data.days.get(&date).cloned().unwrap_or_default();

    Ok(Json(to_response(date, counts)))
}

pub async fn get_stats(State(state): State<AppState>) -> Result<Json<StatsResponse>, AppError> {
    let data = state.data.lock().await;
    Ok(Json(build_stats(&data)))
}

pub async fn click(
    State(state): State<AppState>,
    Json(payload): Json<ClickRequest>,
) -> Result<Json<DailyCountsResponse>, AppError> {
    let action = payload.action.trim();
    if action != "add" && action != "sub" {
        return Err(AppError::bad_request("action must be 'add' or 'sub'"));
    }

    let response = apply_click(&state, action).await?;
    Ok(Json(response))
}

pub async fn click_add(State(state): State<AppState>) -> Result<Redirect, AppError> {
    apply_click(&state, "add").await?;
    Ok(Redirect::to("/"))
}

pub async fn click_sub(State(state): State<AppState>) -> Result<Redirect, AppError> {
    apply_click(&state, "sub").await?;
    Ok(Redirect::to("/"))
}

async fn apply_click(state: &AppState, action: &str) -> Result<DailyCountsResponse, AppError> {
    let date = today_string();
    let mut data = state.data.lock().await;
    let updated = {
        let entry = data.days.entry(date.clone()).or_default();
        if action == "add" {
            entry.add = entry.add.saturating_add(1);
        } else {
            entry.sub = entry.sub.saturating_add(1);
        }
        entry.clone()
    };

    persist_data(&state.data_path, &data).await?;

    Ok(to_response(date, updated))
}

fn to_response(date: String, counts: DayCounts) -> DailyCountsResponse {
    DailyCountsResponse {
        net: counts.add as i64 - counts.sub as i64,
        date,
        add_count: counts.add,
        sub_count: counts.sub,
    }
}

fn today_string() -> String {
    Local::now().date_naive().to_string()
}
