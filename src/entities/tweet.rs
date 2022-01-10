use chrono::{DateTime, Utc};

pub struct Tweet {
    id: Option<i32>,
    pub message: String,
    pub posted_at: DateTime<Utc>,
    pub posted_by: i32,
    deleted: bool,
}

impl Tweet {
    pub fn new(id: i32, message: String, posted_at: DateTime<Utc>, posted_by: i32) -> Tweet {
        Tweet {
            id: Some(id),
            message,
            posted_at,
            posted_by,
            deleted: false,
        }
    }

    pub fn create(message: &str, posted_by: i32) -> Tweet {
        Tweet {
            id: None,
            message: message.into(),
            posted_at: Utc::now(),
            posted_by,
            deleted: false,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    pub fn delete(&mut self) {
        self.deleted = true;
    }
}
