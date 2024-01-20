pub mod auth {
    use chrono::{DateTime, Utc};

    pub struct User {
        id: u32,
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
        fn new(
            id: u32, 
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

        fn get_id(&self) -> &u32{
            &self.id
        }

        fn get_username(&self) -> &str {
            &self.username
        }

        fn get_email(&self) -> &str {
            &self.email
        }

        fn get_password(&self) -> &str {
            &self.password
        }

        fn get_first_name(&self) -> &str {
            &self.first_name
        }

        fn get_last_name(&self) -> &str {
            &self.last_name
        }

        fn get_is_active(&self) -> &Option<bool> {
            &self.is_active
        }

        fn get_is_staff(&self) -> &Option<bool> {
            &self.is_staff
        }

        fn get_is_superuser(&self) -> &Option<bool> {
            &self.is_superuser
        }

        fn get_last_login(&self) -> &Option<DateTime<Utc>> {
            &self.last_login
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
            &self.title.replace_range(.., title);
            self
        }

        fn set_description(&mut self, description: &str) -> &Task {
            &self.description.replace_range(.., description);
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
