#[macro_use]
extern crate lazy_static;
extern crate djangohashers;

mod storage;
mod pg_client;
mod models;

use std::backtrace;
use std::borrow::Borrow;
use std::io::{self, stdout, Read, Write};

use djangohashers::{make_password, check_password};

use models::auth::User;
use models::task::Task;
use storage::user_storage::STORAGE;
use tokio::sync::SemaphorePermit;

#[tokio::main]
async fn main() {
    let _ = pg_client::connect().await;
    let mut auth_menu_loop = true;

    println!("STUDY BUDDY");

    while auth_menu_loop {
        let mut choice_input = String::new();
        let mut auth_status = false;

        show_auth_menu();
        io::stdin().read_line(&mut choice_input).expect("Failed to read line");
        print!("\x1B[2J");

        choice_input = choice_input.trim().to_string();

        match choice_input.as_str() {
            "1" => {
                auth_status = authenticate_user().await;
            }
            _ => println!("You dumass")
        }

        if auth_status {
            auth_menu_loop = false;
        }
    }
}

fn show_auth_menu() {
    let mut lock = stdout().lock();
    let auth_login_screen = 
    r#"
    | LOGIN / REGISTER |
        [1] Login
        [2] Register
        [3] Exit
    "#;

    println!("{}", auth_login_screen);
    write!(lock, ">> ").unwrap();
    io::stdout().flush().unwrap();
}

async fn authenticate_user() -> bool {
    println!("Authenticating...");
    let status = pg_client::auth::validate_login("admin@fake.com", "admin").await.unwrap();

    if status {
        println!("User authenticated!");
    } else {
        println!("Try again...");
    }

    status
}