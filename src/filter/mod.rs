use std::fmt::Display;

use chrono::NaiveDateTime;

pub fn datetime<T: Display>(s: T, _: &dyn askama::Values) -> askama::Result<String> {
    match NaiveDateTime::parse_from_str(&s.to_string(), "%Y-%m-%d %H:%M:%S") {
        Ok(dt) => Ok(dt.format("%B %d, %Y at %-I:%M %p").to_string()),
        Err(_) => Ok("Unknown Date and Time".to_string()),
    }
}
