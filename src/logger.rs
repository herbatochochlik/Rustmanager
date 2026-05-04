use crate::filesystem;
use chrono::Utc;

pub enum LogType {
    SYSTEM(String),
    AUTH(String),
    FILESYS(String),
}

pub fn new_log(log_type: LogType) {
    let mut log_msg: String;
    let now = Utc::now();

    match log_type {
        LogType::SYSTEM(msg) => log_msg = format!("[SYSTEM] {} <{}>\n", msg, now),
        LogType::AUTH(msg) => log_msg = format!("[AUTH] {} <{}>\n", msg, now),
        LogType::FILESYS(msg) => log_msg = format!("[FILESYS] {} <{}>\n", msg, now),
    }

    filesystem::edit_file("upload/global.logs", log_msg);
}
