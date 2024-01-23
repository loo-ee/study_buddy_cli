#[macro_use]
extern crate lazy_static;

mod storage;
mod pg_client;
mod models;

use std::borrow::Borrow;
use models::auth::User;
use models::task::Task;
use storage::user_storage::STORAGE;

fn main() {
    println!("STUDY BUDDY");
    let users = pg_client::get_all_users();

    for user in  users.iter() {
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
