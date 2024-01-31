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

    println!("STUDY BUDDY");

    let is_autheticated = show_auth_menu().await;
}

async fn show_auth_menu() -> bool {
    let mut choice_input = String::new();
    let mut auth_status = false;
    let mut lock = stdout().lock();
    let auth_login_screen = 
    r#"
    | LOGIN / REGISTER |
        [1] Login
        [2] Register
        [3] Exit
    "#;

    loop {
        println!("{}", auth_login_screen);
        write!(lock, ">> ").unwrap();
        io::stdout().flush().unwrap();

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
            break;
        }
    }

    return auth_status;
}

async fn authenticate_user() -> bool {
    let mut tries = 0;
    let mut status: bool = false;
    const MAX_TRIES: u32 = 5;

    while tries < MAX_TRIES {
        status = pg_client::auth::validate_login("admin@fake.com", "admin").await.unwrap();

        println!("Authenticating...");

        if status {
            println!("User authenticated!");
        } else {
            tries += 1;
            println!("Try again...");
        }
    }

    status
}