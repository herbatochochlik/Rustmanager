use crate::logger::{self, LogType};
use crate::names;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn add_file(dir: &str, name: &str) {
    let mut path = PathBuf::from(dir);
    let uniq_name = names::uniq_file_name(dir, name);
    path.push(&uniq_name);

    File::create(&path).expect("Błąd tworzenia");
    logger::new_log(LogType::FILESYS(format!(
        "Added file '{}' in '{}'",
        uniq_name, dir
    )));
}

pub fn add_folder(dir: &str, name: &str) {
    let mut path = PathBuf::from(dir);
    let uniq_name = names::uniq_folder_name(dir, name);
    path.push(&uniq_name);
    fs::create_dir(&path).expect("Błąd tworzenia");
    logger::new_log(LogType::FILESYS(format!(
        "Added folder '{}' in '{}'",
        uniq_name, dir
    )));
}

pub fn edit_file(filedir: &str, input: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filedir)
        .expect("Błąd otwarcia pliku");

    file.write_all(input.as_bytes()).expect("Błąd zapisu");
}
