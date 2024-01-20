use postgres::{Client, NoTls, Error};
use std::sync::Mutex;

lazy_static! {
    pub static ref PG_CLIENT: Mutex<Client> =
        Mutex::new(Client::connect("postgresql://louie:dbpostgres23@localhost/study_buddy", NoTls).expect("Could not connect to server"));
}

pub mod pgclient {

}