pub fn get_timestamp_now_nano() -> i64 {
    chrono::offset::Utc::now().timestamp_micros()
}
pub fn get_timestamp_now_ms() -> i64 {
    chrono::offset::Utc::now().timestamp_millis()
}

pub fn get_human_readable_nano(then: i64) -> String {
    let now = get_timestamp_now_nano();
    let diff_seconds = (then - now) / 1000000;
    let mut diff = timediff::TimeDiff::to_diff(format!("{}s", diff_seconds));
    diff.parse().expect("time diff fail lol")
}
