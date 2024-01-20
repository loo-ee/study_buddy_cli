#[macro_use]
extern crate lazy_static;


mod models;
mod storage;
mod postgres_client;

use std::borrow::Borrow;

use models::auth::User;
use models::task::Task;
use storage::user_storage::STORAGE;

fn main() {
    println!("STUDY BUDDY");

    let mut client = postgres_client::PG_CLIENT.lock().unwrap();
    let rows = client.query("SELECT * FROM project_auth_user", &[]).unwrap(); 

    for row in rows {
        let id: i64 = row.get("id");
        let email: &str = row.get("email");
        println!("{} -> {}", id, email);
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
