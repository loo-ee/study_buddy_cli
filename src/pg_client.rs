use postgres::{Client, NoTls};
use std::sync::Mutex;
use std::{error, env};

use crate::models::auth::User;
use crate::models::auth::serializer::postgres_to_user;

lazy_static! {
    static ref PG_CLIENT: Mutex<Client> = {
        dotenv::dotenv().ok();
        let db_url = 
            match env::var("DATABASE_URL") {
                Ok(val) => val,
                Err(_e) => {
                    println!("DATABASE_URL is not set");
                    panic!("Database url not set");
                }
            };
        
        return Mutex::new(Client::connect(db_url.as_str(), NoTls).
            expect("Could not connect to server"));
    };
}

pub fn get_all_users() -> Vec<User> {
    let mut users: Vec<User> = Vec::new();

    let mut client = PG_CLIENT.lock().unwrap();
    let rows = client.query("SELECT * FROM project_auth_user", &[]).unwrap();

    for row in rows {
        let user: Result<User, Box<dyn error::Error>> = postgres_to_user(row); 
        users.push(user.unwrap());
    }
    users
}
