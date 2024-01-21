pub mod auth {
    use chrono::{DateTime, Utc};

    #[derive(Debug)]
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
        last_login: Option<DateTime<Utc>>,
    }

    impl User {
        pub fn new(
            id: i64, 
            username: String, 
            email: String,
            password: String,
            first_name: String,
            last_name: String,
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
    
    pub mod serializer {
        use std::error::Error;

        use chrono::{DateTime, Utc};
        use postgres::Row;
        use crate::models::auth::User;

        pub fn postgres_to_user(row: Row) -> Result<User, Box<dyn Error>> {
            let id: i64 = row.get("id");
            let username: &str = row.get("username");
            let email: &str = row.get("email");
            let password : &str = row.get("password");
            let first_name: &str = row.get("first_name");
            let last_name: &str = row.get("last_name");
            let is_superuser: bool = row.get("is_superuser");
            let is_staff: bool = row.get("is_staff");
            let is_active: bool = row.get("is_active");
            // TODO: IMPLEMENT DATETIME

            let mut user = User::new(
                id,
                username.to_string(), 
                email.to_string(), 
                password.to_string(), 
                first_name.to_string(), 
                last_name.to_string(), 
            );

            user.set_is_superuser(is_superuser)
                .set_is_active(is_active)
                .set_is_staff(is_staff);
            
            Ok(user) 
        }
    }
}

pub mod task {
    use chrono::{DateTime, Utc};

    pub struct Task {
        title: String,
        description: String,
        date_created: DateTime<Utc>,
    }

    impl Task {
        fn new(&self, title: String, description: String, date_created: DateTime<Utc>) -> Task {
            Task {
                title, description, date_created
            }
        }

        fn set_title(&mut self, title: &str) -> &Task {
            let _ = &self.title.replace_range(.., title);
            self
        }

        fn set_description(&mut self, description: &str) -> &Task {
            let _ = &self.description.replace_range(.., description);
            self
        }

        fn get_title(&self) -> &str {
            &self.title
        }

        fn get_description(&self) -> &str {
            &self.description
        }

        fn get_date_created(&self) -> &DateTime<Utc> {
            &self.date_created
        }
    }
}
