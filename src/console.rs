use crate::filesystem;
use crate::logger::{self, LogType};
use crate::structure::{self, Option};
use std::io::stdin;

pub fn init() {
    println!("Initializing Console User Interface!");
    println!("Welcome to RustManager!");

    structure::check_basic_structure(Option::FULL);

    logger::new_global_log(LogType::SYSTEM(String::from("Initialized CUI")));

    choose_command();
}

fn choose_command() {
    println!(" [0] Add folder \n [1] Add file \n [2] Change directory");

    let input = take_input("What do you want to do?");

    match input.trim() {
        "0" => console_add_folder(),
        "1" => println!("Adding file"),
        "2" => println!("Changing directory"),
        _ => println!("Invalid choice"),
    }
}

fn console_add_folder() {
    let name = take_input("Name the folder: ");

    filesystem::add_folder("upload", &name);
    println!("Added folder: {name}");

    choose_command();
}

fn take_input(msg: &str) -> String {
    println!("{msg}");

    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("Błąd odczytu");

    let trimmed: &str = input.trim();

    return trimmed.to_string();
}
