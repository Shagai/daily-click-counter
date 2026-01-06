use crate::models::{AppData, DailyPoint, StatsResponse, WeeklyAveragePoint, WeeklyPoint};
use chrono::{Datelike, Duration, Local, NaiveDate};

pub fn build_stats(data: &AppData) -> StatsResponse {
    build_stats_at(Local::now().date_naive(), data)
}

pub fn build_stats_at(today: NaiveDate, data: &AppData) -> StatsResponse {
    const WEEK_COUNT: usize = 8;

    let mut last_7_days = Vec::with_capacity(7);
    for offset in (0..7).rev() {
        let date = today - Duration::days(offset as i64);
        let counts = data.days.get(&date_key(date)).cloned().unwrap_or_default();
        last_7_days.push(DailyPoint {
            date: date.to_string(),
            add_count: counts.add,
            sub_count: counts.sub,
            net: counts.add as i64 - counts.sub as i64,
        });
    }

    let current_week_start = week_start(today);
    let mut weekly_totals = Vec::with_capacity(WEEK_COUNT);
    let mut weekly_averages = Vec::with_capacity(WEEK_COUNT);

    for offset in (0..WEEK_COUNT).rev() {
        let start = current_week_start - Duration::weeks(offset as i64);
        let end = start + Duration::days(6);

        let mut add_sum = 0u64;
        let mut sub_sum = 0u64;
        for day_offset in 0..7 {
            let date = start + Duration::days(day_offset);
            let counts = data.days.get(&date_key(date)).cloned().unwrap_or_default();
            add_sum = add_sum.saturating_add(counts.add);
            sub_sum = sub_sum.saturating_add(counts.sub);
        }

        let net = add_sum as i64 - sub_sum as i64;
        let days_counted = if today < start {
            0
        } else if today > end {
            7
        } else {
            (today - start).num_days() as u8 + 1
        };

        let denom = if days_counted == 0 { 1.0 } else { f64::from(days_counted) };

        weekly_totals.push(WeeklyPoint {
            week: week_label(start),
            start_date: start.to_string(),
            end_date: end.to_string(),
            add_count: add_sum,
            sub_count: sub_sum,
            net,
        });

        weekly_averages.push(WeeklyAveragePoint {
            week: week_label(start),
            days_counted,
            avg_add: add_sum as f64 / denom,
            avg_sub: sub_sum as f64 / denom,
            avg_net: net as f64 / denom,
        });
    }

    StatsResponse {
        last_7_days,
        weekly_totals,
        weekly_averages,
    }
}

fn date_key(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

fn week_start(date: NaiveDate) -> NaiveDate {
    date - Duration::days(date.weekday().num_days_from_monday() as i64)
}

fn week_label(date: NaiveDate) -> String {
    let iso = date.iso_week();
    format!("{}-W{:02}", iso.year(), iso.week())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_last_7_days_includes_each_day() {
        let mut data = AppData::default();
        let today = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap();
        let two_days_ago = today - Duration::days(2);
        data.days.insert(
            two_days_ago.to_string(),
            crate::models::DayCounts { add: 3, sub: 1 },
        );

        let stats = build_stats_at(today, &data);
        assert_eq!(stats.last_7_days.len(), 7);
        let point = stats
            .last_7_days
            .iter()
            .find(|day| day.date == two_days_ago.to_string())
            .expect("missing day");
        assert_eq!(point.add_count, 3);
        assert_eq!(point.sub_count, 1);
        assert_eq!(point.net, 2);
    }

    #[test]
    fn stats_weekly_series_lengths() {
        let data = AppData::default();
        let today = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap();
        let stats = build_stats_at(today, &data);
        assert_eq!(stats.weekly_totals.len(), 8);
        assert_eq!(stats.weekly_averages.len(), 8);
        assert_eq!(stats.last_7_days.len(), 7);
    }
}
