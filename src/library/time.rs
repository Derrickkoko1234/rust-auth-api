use chrono::{DateTime, Datelike, Duration, SecondsFormat, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now()-> DateTime<Utc>{
    Utc::now()
}

pub fn now_iso(time: DateTime<Utc>)->String{
    // e.g value "2024-02-02T20:46:44.098Z". equivalent form of JS `new Date().toISOString()``
    time.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn timestamp_millis()->String{
    let now = Utc::now().timestamp_millis();
    format!("{:?}",now)
}

// adds seconds to current time
pub fn add_seconds(seconds:&i64)->DateTime<Utc>{
    Utc::now() + Duration::seconds(seconds.to_owned())
}

pub fn get_nstime() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // The correct way to calculate the current time is
    // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
    // But this is faster, and the difference in terms of entropy is
    // negligible (log2(10^9) == 29.9).
    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

pub fn get_current_date_year_and_month() -> (String, usize) {
    let now = Utc::now();
    let year = now.year().to_string();
    let month = now.month();
    
    (year, month as usize)
}