use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn add_file(dir: &str, name: &str, inside: &[u8]) {
    let path = uniq_file_name(dir, name);
    let mut file = File::create(&path).expect("Błąd tworzenia");

    file.write_all(inside).expect("Błąd zapisu");
}

pub fn uniq_file_name(dir: &str, name: &str) -> PathBuf {
    let mut path = PathBuf::from(dir);
    path.push(name);

    let mut i: i8 = 2;

    while path.exists() {
        let mut temp_arr: Vec<String> = name.split('.').map(|s| s.to_string()).collect();
        temp_arr[0] = format!("{} ({})", temp_arr[0], i);
        let temp_name = format!("{}.{}", temp_arr[0], temp_arr[1]);
        path = PathBuf::from(dir);
        path.push(temp_name);

        i += 1;
    }

    return path;
}
