use crate::logger::{self, LogType};

pub fn init() {
    println!("Initializing Graphical User Interface!");
    println!("Welcome to RustManager!");

    logger::new_global_log(LogType::SYSTEM(String::from("Initialized GUI")));
}
