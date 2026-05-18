use crate::logger::{self, LogType};
use mongodb::Client;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use std::fs::{self, File};
use std::path::{self, PathBuf};

pub enum Option {
    FULL,
    SYSTEM(SystemOption),
    USERS(UsersOption),
}

pub enum SystemOption {
    FULL,
    LOGS,
    CONFIG,
}

pub enum UsersOption {
    FULL,
    ADMIN(UserOption),
}

pub enum UserOption {
    FULL,
    LOGS,
    CONFIG,
    FILES,
}

pub fn check_basic_structure(options: Option) {
    match options {
        Option::FULL => {
            if !does_it_exist("upload") {
                repair_basic_structure(Option::FULL);
            } else {
                check_basic_structure(Option::SYSTEM(SystemOption::FULL));
                check_basic_structure(Option::USERS(UsersOption::FULL));
            }
        }
        Option::SYSTEM(suboption) => match suboption {
            SystemOption::FULL => {
                if !does_it_exist("upload/SYSTEM") {
                    repair_basic_structure(Option::SYSTEM(SystemOption::FULL));
                } else {
                    check_basic_structure(Option::SYSTEM(SystemOption::LOGS));
                    check_basic_structure(Option::SYSTEM(SystemOption::CONFIG));
                }
            }
            SystemOption::CONFIG => {
                if !does_it_exist("upload/SYSTEM/global.config") {
                    repair_basic_structure(Option::SYSTEM(SystemOption::CONFIG));
                }
            }
            SystemOption::LOGS => {
                if !does_it_exist("upload/SYSTEM/global.logs") {
                    repair_basic_structure(Option::SYSTEM(SystemOption::LOGS));
                }
            }
        },
        Option::USERS(suboption) => match suboption {
            UsersOption::FULL => {
                if !does_it_exist("upload/USERS") {
                    repair_basic_structure(Option::USERS(UsersOption::FULL));
                } else {
                    check_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::FULL)));
                }
            }
            UsersOption::ADMIN(option) => match option {
                UserOption::FULL => {
                    if !does_it_exist("upload/USERS/Admin") {
                        repair_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::FULL)));
                    } else {
                        check_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::LOGS)));
                        check_basic_structure(Option::USERS(UsersOption::ADMIN(
                            UserOption::CONFIG,
                        )));
                        check_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::FILES)));
                    }
                }
                UserOption::CONFIG => {
                    if !does_it_exist("upload/USERS/Admin/local.config") {
                        repair_basic_structure(Option::USERS(UsersOption::ADMIN(
                            UserOption::CONFIG,
                        )));
                    }
                }
                UserOption::LOGS => {
                    if !does_it_exist("upload/USERS/Admin/local.logs") {
                        repair_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::LOGS)));
                    }
                }
                UserOption::FILES => {
                    if !does_it_exist("upload/USERS/Admin/FILES") {
                        repair_basic_structure(Option::USERS(UsersOption::ADMIN(
                            UserOption::FILES,
                        )));
                    }
                }
            },
        },
    }
}

pub fn does_it_exist(dir: &str) -> bool {
    let path = PathBuf::from(dir);

    return path.exists();
}

fn repair_basic_structure(options: Option) {
    match options {
        Option::FULL => {
            fs::create_dir("upload").expect("Błąd tworzenia");
            repair_basic_structure(Option::SYSTEM(SystemOption::FULL));
            repair_basic_structure(Option::USERS(UsersOption::FULL));

            logger::new_log(LogType::FILESTRUCT(String::from(
                "Finished repairing FULL filestructure",
            )));
        }
        Option::SYSTEM(suboption) => match suboption {
            SystemOption::FULL => {
                fs::create_dir("upload/SYSTEM").expect("Błąd tworzenia");
                repair_basic_structure(Option::SYSTEM(SystemOption::LOGS));
                repair_basic_structure(Option::SYSTEM(SystemOption::CONFIG));
                logger::new_log(LogType::FILESTRUCT(String::from(
                    "Finished repairing SYSTEM filestructure",
                )));
            }
            SystemOption::CONFIG => {
                let mut path = PathBuf::from("upload/SYSTEM");
                path.push("global.config");
                File::create(&path).expect("Błąd tworzenia");
                logger::new_log(LogType::FILESTRUCT(String::from(
                    "Finished repairing SYSTEM CONFIG filestructure",
                )));
            }
            SystemOption::LOGS => {
                let mut path = PathBuf::from("upload/SYSTEM");
                path.push("global.logs");
                File::create(&path).expect("Błąd tworzenia");
                logger::new_log(LogType::FILESTRUCT(String::from(
                    "Finished repairing SYSTEM LOGS filestructure",
                )));
            }
        },
        Option::USERS(suboption) => match suboption {
            UsersOption::FULL => {
                fs::create_dir("upload/USERS").expect("Błąd tworzenia");
                repair_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::FULL)));
                logger::new_log(LogType::FILESTRUCT(String::from(
                    "Finished repairing USERS filestructure",
                )));
            }
            UsersOption::ADMIN(option) => match option {
                UserOption::FULL => {
                    fs::create_dir("upload/USERS/Admin").expect("Błąd tworzenia");
                    repair_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::LOGS)));
                    repair_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::CONFIG)));
                    repair_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::FILES)));
                    logger::new_log(LogType::FILESTRUCT(String::from(
                        "Finished repairing USERS ADMIN filestructure",
                    )));
                }
                UserOption::LOGS => {
                    let mut path = PathBuf::from("upload/USERS/Admin");
                    path.push("local.logs");
                    File::create(&path).expect("Błąd tworzenia");
                    logger::new_log(LogType::FILESTRUCT(String::from(
                        "Finished repairing USERS ADMIN LOGS filestructure",
                    )));
                }
                UserOption::CONFIG => {
                    let mut path = PathBuf::from("upload/USERS/Admin");
                    path.push("local.config");
                    File::create(&path).expect("Błąd tworzenia");
                    logger::new_log(LogType::FILESTRUCT(String::from(
                        "Finished repairing USERS ADMIN CONFIG filestructure",
                    )));
                }
                UserOption::FILES => {
                    fs::create_dir("upload/USERS/Admin/FILES").expect("Błąd tworzenia");
                    logger::new_log(LogType::FILESTRUCT(String::from(
                        "Finished repairing USERS ADMIN FILES filestructure",
                    )));
                }
            },
        },
    }
}

pub fn check_user_filestructure(folder_id: &str, options: UserOption) {
    check_basic_structure(Option::USERS(UsersOption::FULL));

    match options {
        UserOption::FULL => {
            let path = format!("upload/USERS/{}", &folder_id);
            if !does_it_exist(&path) {
                repair_user_structure(folder_id, UserOption::FULL);
            } else {
                check_user_filestructure(folder_id, UserOption::LOGS);
                check_user_filestructure(folder_id, UserOption::CONFIG);
                check_user_filestructure(folder_id, UserOption::FILES);
            }
        }
        UserOption::LOGS => {
            let path = format!("upload/USERS/{}/local.logs", &folder_id);
            if !does_it_exist(&path) {
                repair_user_structure(folder_id, UserOption::LOGS);
            }
        }
        UserOption::CONFIG => {
            let path = format!("upload/USERS/{}/local.config", &folder_id);
            if !does_it_exist(&path) {
                repair_user_structure(folder_id, UserOption::CONFIG);
            }
        }
        UserOption::FILES => {
            let path = format!("upload/USERS/{}/FILES", &folder_id);
            if !does_it_exist(&path) {
                repair_user_structure(folder_id, UserOption::FILES);
            }
        }
    }
}

fn repair_user_structure(folder_id: &str, options: UserOption) {
    match options {
        UserOption::FULL => {
            let path = format!("upload/USERS/{}", &folder_id);
            fs::create_dir(&path).expect("Błąd tworzenia");
            repair_user_structure(folder_id, UserOption::LOGS);
            repair_user_structure(folder_id, UserOption::CONFIG);
            repair_user_structure(folder_id, UserOption::FILES);
            logger::new_log(LogType::FILESTRUCT(format!(
                "Finished repairing USERS filestructure for user: {}",
                &folder_id
            )));
        }
        UserOption::LOGS => {
            let mut path = PathBuf::from(format!("upload/USERS/{}", &folder_id));
            path.push("local.logs");
            File::create(&path).expect("Błąd tworzenia");
            logger::new_log(LogType::FILESTRUCT(format!(
                "Finished repairing USERS LOGS filestructure for user: {}",
                &folder_id
            )));
        }
        UserOption::CONFIG => {
            let mut path = PathBuf::from(format!("upload/USERS/{}", &folder_id));
            path.push("local.config");
            File::create(&path).expect("Błąd tworzenia");
            logger::new_log(LogType::FILESTRUCT(format!(
                "Finished repairing USERS CONFIG filestructure for user: {}",
                &folder_id
            )));
        }
        UserOption::FILES => {
            let path = format!("upload/USERS/{}/FILES", &folder_id);
            fs::create_dir(&path).expect("Błąd tworzenia");
            logger::new_log(LogType::FILESTRUCT(format!(
                "Finished repairing USERS FILES filestructure for user: {}",
                &folder_id
            )));
        }
    }
}
