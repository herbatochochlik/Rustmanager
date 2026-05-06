use crate::filesystem;
use chrono::Utc;

pub enum LogType {
    SYSTEM(String),
    AUTH(String),
    FILESYS(String),
}

pub fn new_log(log_type: LogType) {
    let log_msg: String;
    let now = Utc::now();

    match log_type {
        LogType::SYSTEM(msg) => log_msg = format!("{} [SYSTEM] {}\n", now, msg),
        LogType::AUTH(msg) => log_msg = format!("{} [AUTH] {}\n", now, msg),
        LogType::FILESYS(msg) => log_msg = format!("{} [FILESYS] {}\n", now, msg),
    }

    filesystem::edit_file("upload/global.logs", log_msg);
}
