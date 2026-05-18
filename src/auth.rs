use mongodb::{
    Client,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use serde::{Deserialize, Serialize}; // Dodaj Serialize dla kompletności

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

    let filter = doc! {"login": username};

    match users.find_one(filter).await {
        Ok(Some(doc)) => {
            if doc.password == password {
                return AUTH::SUCCESS(doc.folder_id);
            } else {
                return AUTH::FAIL(String::from("Password is incorrect"));
            }
        }
        Ok(None) => AUTH::FAIL(String::from("This user does not exist")),
        Err(e) => {
            eprintln!("Błąd bazy danych: {}", e);
            return AUTH::FAIL(String::from("Database error"));
        }
    }
}
