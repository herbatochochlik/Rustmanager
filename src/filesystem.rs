use crate::logger::{self, LogType};
use crate::names;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

pub fn add_file(dir: &str, name: &str) {
    let path = names::uniq_file_name(dir, name);
    File::create(&path).expect("Błąd tworzenia");
    logger::new_log(LogType::FILESYS(format!("Added {} file in {}", name, dir)));
}

pub fn add_folder(dir: &str, name: &str) {
    let path = names::uniq_folder_name(dir, name);
    fs::create_dir(path).expect("Błąd tworzenia");
    logger::new_log(LogType::FILESYS(format!(
        "Added {} folder in {}",
        name, dir
    )));
}

pub fn edit_file(filedir: &str, input: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filedir)
        .expect("Błąd otwarcia pliku");

    file.write_all(input.as_bytes()).expect("Błąd zapisu");
}
