pub fn get_timestamp_now() -> i64 {
    chrono::offset::Utc::now().timestamp_micros()
}