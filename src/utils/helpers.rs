use chrono::{DateTime, Utc};
use std::time::SystemTime;

pub fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn format_duration(duration: &chrono::Duration) -> String {
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    if days > 0 {
        format!("{} days, {} hours", days, hours)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    }
}

pub fn extract_resource_id(arn: &str) -> crate::utils::error::Result<String> {
    let parts: Vec<&str> = arn.split(':').collect();
    if parts.len() >= 6 {
        let resource_part = parts[5];
        if let Some(id) = resource_part.split('/').last() {
            Ok(id.to_string())
        } else {
            Ok(resource_part.to_string())
        }
    } else {
        Err(crate::utils::error::AppError::Parse(format!(
            "Invalid ARN format: {}",
            arn
        )))
    }
}

pub fn extract_region_from_arn(arn: &str) -> crate::utils::error::Result<String> {
    let parts: Vec<&str> = arn.split(':').collect();
    if parts.len() >= 4 {
        Ok(parts[3].to_string())
    } else {
        Err(crate::utils::error::AppError::Parse(format!(
            "Invalid ARN format: {}",
            arn
        )))
    }
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn system_time_to_datetime(time: SystemTime) -> DateTime<Utc> {
    DateTime::from(time)
}
