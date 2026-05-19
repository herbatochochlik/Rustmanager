use crate::filesystem;
use chrono::Utc;

pub enum LogType {
    SYSTEM(String),
    AUTH(String),
    FILESYS(String),
    FILESTRUCT(String),
}

pub fn new_global_log(log_type: LogType) {
    let log_msg: String;
    let now = Utc::now();

    match log_type {
        LogType::SYSTEM(msg) => log_msg = format!("{} [SYSTEM] {}\n", now, msg),
        LogType::AUTH(msg) => log_msg = format!("{} [AUTH] {}\n", now, msg),
        LogType::FILESYS(msg) => log_msg = format!("{} [FILESYS] {}\n", now, msg),
        LogType::FILESTRUCT(msg) => log_msg = format!("{} [FILESTRUCT] {}\n", now, msg),
    }

    filesystem::edit_file("upload/SYSTEM/global.logs", log_msg);
}

pub fn new_local_log(folder_id: &str, log_type: LogType) {
    let log_msg: String;
    let now = Utc::now();

    let path = format!("upload/USERS/{}/local.logs", folder_id);
    
    match log_type {
        LogType::SYSTEM(msg) => log_msg = format!("{} [SYSTEM] {}\n", now, msg),
        LogType::AUTH(msg) => log_msg = format!("{} [AUTH] {}\n", now, msg),
        LogType::FILESYS(msg) => log_msg = format!("{} [FILESYS] {}\n", now, msg),
        LogType::FILESTRUCT(msg) => log_msg = format!("{} [FILESTRUCT] {}\n", now, msg),
    }

    filesystem::edit_file(&path, log_msg);
}
