use crate::filesystem;

pub enum LogType {
    SYSTEM(String),
    AUTH(String),
    FILESYS(String),
}

pub fn new_log(log_type: LogType) {
    
}
