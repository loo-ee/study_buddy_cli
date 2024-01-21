pub mod pgclient {
    use postgres::{Client, NoTls};
    use std::sync::Mutex;
    use std::error;

    use crate::models::auth::User;
    use crate::models::auth::serializer::postgres_to_user;

    lazy_static! {
        static ref PG_CLIENT: Mutex<Client> =
            Mutex::new(Client::connect("postgresql://louie:dbpostgres23@localhost/study_buddy", NoTls).expect("Could not connect to server"));
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
}