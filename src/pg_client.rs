use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::sync::Mutex;
use std::env;
use tokio::task::JoinHandle;

use crate::models::auth::User;

lazy_static! {
    static ref PG_CLIENT: Mutex<Option<PgPool>> = Mutex::new(None);
    static ref DB_URL: String = {
        dotenv::dotenv().ok();
        let db_url = String::from( 
            match env::var("DATABASE_URL") {
                Ok(val) => val,
                Err(_e) => {
                    println!("DATABASE_URL is not set");
                    panic!("Database url not set");
                }
            });

        db_url
    };
}

pub fn connect() -> JoinHandle<Result<(), sqlx::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&DB_URL);

    tokio::spawn(async move {
        let pool = pool.await?;
        let mut pool_guard = PG_CLIENT.lock().unwrap();
        *pool_guard = Some(pool);
        Ok(())
    })
}

pub mod auth {
    use sqlx::Row;

    use super::PG_CLIENT;

    pub async fn validate_login(email: &str, password: &str) -> Result<bool, sqlx::Error> {
        let pool_guard = PG_CLIENT.lock().unwrap();

        match &*pool_guard {
            Some(pool) => {
                let row = sqlx::query("SELECT password FROM project_auth_user WHERE email=$1")
                    .bind(email)
                    .fetch_one(pool)
                    .await?;

                let user_hashed_pass = row.get::<String, _>("password");
                let is_valid = djangohashers::check_password(password, &user_hashed_pass).unwrap();
                Ok(is_valid)
            },
            None => Err(sqlx::Error::Configuration(String::from("Pool not initialized").into())),
        }
    }
}

pub async fn get_all_users() -> Result<Vec<User>, sqlx::Error> {
    let pool_guard = PG_CLIENT.lock().unwrap();

    match &*pool_guard {
        Some(pool) => {
            let users = sqlx::query_as::<_, User>("SELECT * FROM project_auth_user")
                .fetch_all(pool)
                .await?;
            
            Ok(users)
        },
        None => Err(sqlx::Error::Configuration(String::from("Pool not initialized").into())),
    }
}

pub async fn get_one_user(email: &str) -> Result<User, sqlx::Error> {
    let pool_guard = PG_CLIENT.lock().unwrap();

    match &*pool_guard {
        Some(pool) => {
            let found_user = sqlx::query_as::<_, User>("SELECT * FROM project_auth_user WHERE email=$1")
            .bind(email)
            .fetch_one(pool)
            .await?;

            Ok(found_user)
        },
        None => Err(sqlx::Error::Configuration(String::from("Pool not initialized").into())),
    }
}

