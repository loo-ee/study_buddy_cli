#[macro_use]
extern crate lazy_static;
extern crate djangohashers;

mod storage;
mod pg_client;
mod models;

use std::borrow::Borrow;
use djangohashers::{make_password, check_password};
use models::auth::User;
use models::task::Task;
use storage::user_storage::STORAGE;

#[tokio::main]
async fn main() {
    println!("STUDY BUDDY");
    let _ = pg_client::connect().await;
    let is_valid = pg_client::auth::validate_login("admin@fake.com", "admin").await.unwrap();
    println!("Is valid: {:?}", is_valid);

    if is_valid {
        show_menu();
        // let users = pg_client::get_all_users().await;

        // for user in  users.unwrap() {
        //     println!("{:?}", user);
        // }
    }
}

fn show_menu() {
    let auth_login_screen = 
    r#"
    | LOGIN / REGISTER |
        [1] Login
        [2] Register
        [3] Exit
    "#;

    println!("{}", auth_login_screen);
}
