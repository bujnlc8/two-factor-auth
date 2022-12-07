use std::time::SystemTime;

// 获取当前时间戳
pub fn get_current_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
