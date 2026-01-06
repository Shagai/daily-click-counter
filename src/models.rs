use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DayCounts {
    pub add: u64,
    pub sub: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub days: BTreeMap<String, DayCounts>,
}

#[derive(Debug, Deserialize)]
pub struct ClickRequest {
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyCountsResponse {
    pub date: String,
    pub add_count: u64,
    pub sub_count: u64,
    pub net: i64,
}

#[derive(Debug, Serialize)]
pub struct DailyPoint {
    pub date: String,
    pub add_count: u64,
    pub sub_count: u64,
    pub net: i64,
}

#[derive(Debug, Serialize)]
pub struct WeeklyPoint {
    pub week: String,
    pub start_date: String,
    pub end_date: String,
    pub add_count: u64,
    pub sub_count: u64,
    pub net: i64,
}

#[derive(Debug, Serialize)]
pub struct WeeklyAveragePoint {
    pub week: String,
    pub days_counted: u8,
    pub avg_add: f64,
    pub avg_sub: f64,
    pub avg_net: f64,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub last_7_days: Vec<DailyPoint>,
    pub weekly_totals: Vec<WeeklyPoint>,
    pub weekly_averages: Vec<WeeklyAveragePoint>,
}
