use std::sync::Mutex;
use crate::models::auth::User;

lazy_static! {
    pub static ref STORAGE: Mutex<UserStorage> = Mutex::new(UserStorage::new());
}

pub struct UserStorage {
    users: Vec<User>
}

impl UserStorage {
    fn new() -> UserStorage {
        UserStorage { users: vec![]}
    }

    fn insert_user(&mut self, user: User) -> &UserStorage {
        &self.users.push(user);
        self
    }
}