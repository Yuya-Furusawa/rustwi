use crate::entities::{Account, Tweet as TweetEntity};

pub struct Tweet {
    pub id: String,
    pub name: String,
    pub message: String,
    pub posted_at: String,
}

impl From<(TweetEntity, &Account)> for Tweet {
    fn from(e: (TweetEntity, &Account)) -> Self {
        Tweet {
            id: e.0.id().unwrap_or(-1).to_string(),
            name: e.1.display_name.clone(),
            message: e.0.message,
            posted_at: e.0.posted_at.format("%Y/%m/%d %H:%M").to_string(),
        }
    }
}
