use crate::names;
use std::fs::{self, File};
use std::io::Write;

pub fn add_file(dir: &str, name: &str, inside: &[u8]) {
    let path = names::uniq_file_name(dir, name);
    let mut file = File::create(&path).expect("Błąd tworzenia");

    file.write_all(inside).expect("Błąd zapisu");
}

pub fn add_folder(dir: &str, name: &str) {
    let path = names::uniq_folder_name(dir, name);
    fs::create_dir(path).expect("Błąd tworzenia");
}
