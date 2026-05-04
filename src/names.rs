use std::path::PathBuf;

pub fn uniq_file_name(dir: &str, name: &str) -> PathBuf {
    let mut path: PathBuf = PathBuf::from(dir);
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

pub fn uniq_folder_name(dir: &str, name: &str) -> PathBuf {
    let mut path: PathBuf = PathBuf::from(dir);
    path.push(name);

    let mut i: i8 = 2;

    while path.exists() {
        let temp_name = format!("{name} ({i})");
        path = PathBuf::from(dir);
        path.push(temp_name);

        i += 1;
    }
    return path;
}

pub fn does_it_exist(dir: &str) -> bool {
    let mut path = PathBuf::from(dir);

    return path.exists();
}
