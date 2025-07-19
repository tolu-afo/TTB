use chrono::{Duration, NaiveDateTime, Utc};

pub fn relative_time_string(datetime: NaiveDateTime) -> String {
    let now = Utc::now().naive_utc();
    let duration = now.signed_duration_since(datetime);

    if duration < Duration::seconds(60) {
        format!("{} second(s) ago", duration.num_seconds())
    } else if duration < Duration::minutes(60) {
        format!("{} minute(s) ago", duration.num_minutes())
    } else if duration < Duration::hours(24) {
        format!("{} hour(s) ago", duration.num_hours())
    } else if duration < Duration::days(30) {
        format!("{} day(s) ago", duration.num_days())
    } else if duration < Duration::days(365) {
        format!("{} month(s) ago", duration.num_days() / 30)
    } else {
        format!("{} year(s) ago", duration.num_days() / 365)
    }
}

pub fn overflow_add(a: i64, b: i64) -> i64 {
    // calls built-in function checked_add, if returns none, function returns a
    match a.checked_add(b) {
        Some(x) => x,
        None => a,
    }
}

pub fn _overflow_sub(a: i64, b: i64) -> i64 {
    // calls built-in function checked_add, if returns none, function returns a
    match a.checked_sub(b) {
        Some(x) => x,
        None => a,
    }
}
