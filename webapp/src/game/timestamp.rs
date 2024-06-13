pub fn get_timestamp_now_nano() -> i64 {
    chrono::offset::Utc::now().timestamp_micros()
}
pub fn get_timestamp_now_ms() -> i64 {
    chrono::offset::Utc::now().timestamp_millis()
}
