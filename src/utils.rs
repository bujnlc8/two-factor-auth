use std::{
    io::{self, Write},
    time::SystemTime,
};

// 获取当前时间戳
pub fn get_current_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn clear_current_line() {
    // 使用 ANSI 转义序列清除行并将光标移到行首
    print!("\r\x1B[2K");
    io::stdout().flush().unwrap();
}
