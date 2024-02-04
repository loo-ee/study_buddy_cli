#[macro_use]
extern crate lazy_static;
extern crate djangohashers;

mod storage;
mod pg_client;
mod models;

use std::backtrace;
use std::borrow::Borrow;
use std::io::{self, stdout, Read, Write};

use models::auth::User;
use models::task::Task;
use storage::user_storage::STORAGE;
use tokio::sync::SemaphorePermit;

#[tokio::main]
async fn main() {
    let _ = pg_client::connect().await;

    println!("STUDY BUDDY");

    let is_authenticated = show_auth_menu().await;

    if !is_authenticated {
        return;
    }


}

async fn show_auth_menu() -> bool {
    let mut choice_input = String::new();
    let mut auth_status = false;
    let mut lock = stdout().lock();
    let auth_login_screen = 
    r#"
    | LOGIN / REGISTER |
        [1] Login
        [2] Exit
    "#;

    loop {
        println!("{}", auth_login_screen);
        write!(lock, ">> ").unwrap();
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut choice_input).expect("Failed to read line");
        print!("\x1B[2J");

        match choice_input.trim() {
            "1" => {
                auth_status = authenticate_user().await;

                if auth_status {
                    return true
                }
            }
            "2" => {
                break;
            }
            _ => println!("You dumass")
        }

        choice_input.clear();
    }

    return false
}

async fn authenticate_user() -> bool {
    let mut tries = 0;
    let mut status: bool = false;
    let mut email_input = String::new();
    let mut pass_input = String::new();
    let mut lock = stdout().lock();
    const MAX_TRIES: u32 = 5;

    while tries < MAX_TRIES {
        io::stdout().flush().unwrap();
        write!(lock, "Enter email: ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut email_input).expect("Error reading line");

        write!(lock, "Enter password: ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut pass_input).expect("Error reading line");

        status = pg_client::auth::validate_login(email_input.trim(), pass_input.trim()).await.unwrap();
        println!("Authenticating...");

        if status {
            println!("User authenticated!");
            break;
        } else {
            tries += 1;
            email_input.clear();
            pass_input.clear();
            println!("Try again...");
        }
    }

    status
}

async fn show_actions() {
    
}