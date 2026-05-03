use std::env;

mod console;
mod filesystem;
mod graphical;
mod logger;
mod names;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No valid interface chosen");
    } else if args[1] == "graphical" {
        graphical::init();
    } else if args[1] == "console" {
        console::init();
    } else {
        println!("No valid interface chosen");
    }
}
