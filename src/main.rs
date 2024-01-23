#[macro_use]
extern crate lazy_static;

mod storage;
mod pg_client;
mod models;

use std::borrow::Borrow;
use models::auth::User;
use models::task::Task;
use storage::user_storage::STORAGE;

#[tokio::main]
async fn main() {
    println!("STUDY BUDDY");
    let _ = pg_client::connect().await;
    let users = pg_client::get_all_users().await;

    for user in  users.unwrap() {
        println!("{:?}", user);
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
}
