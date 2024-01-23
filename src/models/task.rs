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