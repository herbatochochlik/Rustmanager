use crate::logger::{self, LogType};
use std::fs::{self, File};
use std::path::PathBuf;

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
                    check_basic_structure(Option::SYSTEM(SystemOption::CONFIG));
                    check_basic_structure(Option::SYSTEM(SystemOption::LOGS));
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
                        check_basic_structure(Option::USERS(UsersOption::ADMIN(
                            UserOption::CONFIG,
                        )));
                        check_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::LOGS)));
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
                        repair_basic_structure(Option::USERS(UsersOption::ADMIN(UserOption::FILES)));
                    }
                }
            },
        },
    }
}

pub fn does_it_exist(dir: &str) -> bool {
    let mut path = PathBuf::from(dir);

    return path.exists();
}

fn repair_basic_structure(option: Option) {}
