use std::path::PathBuf;

pub fn uniq_file_name(dir: &str, name: &str) -> String {
    let mut i: i32 = 2;
    let mut temp_name: String = name.to_string();

    let mut path: PathBuf = PathBuf::from(dir);
    path.push(&temp_name);

    while path.exists() {
        let mut temp_arr: Vec<String> = name.split('.').map(|s| s.to_string()).collect();
        temp_arr[0] = format!("{} ({})", temp_arr[0], i);
        temp_name = format!("{}.{}", temp_arr[0], temp_arr[1]);
        path = PathBuf::from(dir);
        path.push(&temp_name);

        i += 1;
    }

    return temp_name;
}

pub fn uniq_folder_name(dir: &str, name: &str) -> String {
    let mut i: i32 = 2;
    let mut temp_name = name.to_string();

    let mut path = PathBuf::from(dir);
    path.push(&temp_name);

    while path.exists() {
        temp_name = format!("{name} ({i})");
        path = PathBuf::from(dir);
        path.push(&temp_name);

        i += 1;
    }

    return temp_name;
}

pub fn does_it_exist(dir: &str) -> bool {
    let mut path = PathBuf::from(dir);

    return path.exists();
}
