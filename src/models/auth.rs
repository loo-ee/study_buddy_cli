use chrono::{DateTime, Utc};

#[derive(Debug)]
#[derive(sqlx::FromRow)]
pub struct User {
    id: i64,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    is_superuser: Option<bool>,
    is_staff: Option<bool>,
    is_active: Option<bool>,
    last_login: Option<chrono::DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: i64, 
        username: String, 
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        last_login: DateTime<Utc>,
    ) -> User {
        User { 
            id, username, email, password, first_name, last_name, 
            is_active: None, is_staff: None, is_superuser: None, last_login: None 
        }
    }

    pub fn get_id(&self) -> &i64{
        &self.id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }

    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }

    pub fn get_is_active(&self) -> &Option<bool> {
        &self.is_active
    }

    pub fn get_is_staff(&self) -> &Option<bool> {
        &self.is_staff
    }

    pub fn get_is_superuser(&self) -> &Option<bool> {
        &self.is_superuser
    }

    pub fn get_last_login(&self) -> &Option<DateTime<Utc>> {
        &self.last_login
    }

    pub fn set_is_superuser(&mut self, status: bool) -> &mut User {
        self.is_superuser = Some(status);
        self
    }

    pub fn set_is_staff(&mut self, status: bool) -> &mut User {
        self.is_staff = Some(status);
        self
    }

    pub fn set_is_active(&mut self, status: bool) ->&mut User {
        self.is_active = Some(status);
        self
    }

    pub fn set_last_login(&mut self, time: DateTime<Utc>) -> &mut User {
        self.last_login = Some(time);
        self
    }
}
