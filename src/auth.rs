use crate::logger::{self, LogType};
use mongodb::{Client, bson::doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserDocument {
    login: String,
    password: String,
    folder_id: String,
}

pub enum AUTH {
    FAIL(String),    //returns fail reason
    SUCCESS(String), //returns folder_id
}

pub async fn authenticate_user(client: Client, username: String, password: String) -> AUTH {
    let users: mongodb::Collection<UserDocument> =
        client.database("rustmanager").collection("users");

    let filter = doc! {"login": &username};

    match users.find_one(filter).await {
        Ok(Some(doc)) => {
            if doc.password == password {
                logger::new_global_log(LogType::AUTH(format!(
                    "User {} successfully logged in",
                    &username
                )));
                logger::new_local_log(
                    &doc.folder_id,
                    LogType::AUTH(String::from("Successfully logged in")),
                );
                return AUTH::SUCCESS(doc.folder_id);
            } else {
                logger::new_global_log(LogType::AUTH(format!(
                    "Failed login attempt by user {}: The password is incorrect",
                    &username
                )));
                logger::new_local_log(
                    &doc.folder_id,
                    LogType::AUTH(String::from(
                        "Failed login attempt: The password is incorrect",
                    )),
                );
                return AUTH::FAIL(String::from("Password is incorrect"));
            }
        }
        Ok(None) => {
            logger::new_global_log(LogType::AUTH(format!(
                "Failed login attempt by user {}: The user does not exist",
                &username
            )));
            AUTH::FAIL(String::from("This user does not exist"))
        }
        Err(e) => {
            eprintln!("Błąd bazy danych: {}", e);
            return AUTH::FAIL(String::from("Database error"));
        }
    }
}
