use std::collections::HashSet;

use crate::entities::Tweet;
use crate::repositories::{Accounts, Tweets};
use crate::request::UserContext;
use crate::views::Home;

pub async fn list_tweets(repo: &impl Tweets, account_repo: &impl Accounts) -> Home {
    let tweets = repo.list().await;
    let posted_account_ids = tweets.iter().map(|x| x.posted_by).collect::<HashSet<i32>>();
    let accounts = account_repo.find(posted_account_ids).await;
    let tweets = tweets
        .into_iter()
        .map(|x| {
            let account = accounts.get(&x.posted_by).unwrap();
            (x, account).into()
        })
        .collect();
    Home { tweets }
}

pub async fn create_tweet(repo: &impl Tweets, user_context: &UserContext, message: &str) {
    let new_tweet = Tweet::create(message, user_context.user_id);
    repo.store(&new_tweet).await;
}

pub async fn delete_tweet(repo: &impl Tweets, id: i32) {
    let tweet = repo.find(id).await;
    if let Some(mut tweet) = tweet {
        tweet.delete();
        repo.store(&tweet).await;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use std::collections::HashMap;

    use crate::entities::{Account, Tweet};
    use crate::repositories::{MockAccounts, MockTweets};
    use crate::request::UserContext;

    fn tweet(id: i32, account_id: i32) -> Tweet {
        Tweet::new(
            id,
            format!("message{}", id),
            Utc.ymd(2020, 1, 1).and_hms(0, 0, 0),
            account_id,
        )
    }

    fn account(id: i32) -> Account {
        Account::new(
            id,
            format!("{}@example.com", id),
            format!("password{}", id),
            format!("display_name{}", id),
        )
    }

    #[tokio::test]
    async fn test_list_tweets() {
        let mut tweets = MockTweets::new();
        tweets
            .expect_list()
            .returning(|| vec![tweet(2, 2), tweet(1, 1)]);

        let mut accounts = MockAccounts::new();
        accounts.expect_find().returning(|_| {
            let mut result = HashMap::new();
            result.insert(1, account(1));
            result.insert(2, account(2));
            result
        });

        let result = super::list_tweets(&tweets, &accounts).await;
        assert_eq!(result.tweets.len(), 2);
        let result0 = result.tweets.get(0).unwrap();
        assert_eq!(result0.message, "message2");
        assert_eq!(result0.posted_at, "2020/01/01 00:00");
        assert_eq!(result0.name, "display_name2");
    }

    #[tokio::test]
    async fn test_list_tweets_empty() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![]);

        let mut accounts = MockAccounts::new();
        accounts.expect_find().returning(|_| HashMap::new());

        let result = super::list_tweets(&tweets, &accounts).await;
        assert_eq!(result.tweets.is_empty(), true);
    }

    #[tokio::test]
    async fn test_create_tweet() {
        let user_context = UserContext { user_id: 1 };

        let mut tweets = MockTweets::new();
        tweets
            .expect_store()
            .withf(|e| e.message == tweet(1, 1).message && e.posted_by == 1)
            .once()
            .return_const(());

        let tweet = tweet(1, 1);
        super::create_tweet(&tweets, &user_context, &tweet.message).await;
    }

    #[tokio::test]
    async fn test_delete_tweet() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| Some(tweet(1, 1)));
        tweets
            .expect_store()
            .withf(|e| e.id() == Some(1) && e.is_deleted())
            .once()
            .return_const(());

        super::delete_tweet(&tweets, 1).await;
    }

    #[tokio::test]
    async fn test_delete_tweet_not_found() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| None);
        tweets.expect_store().never();

        super::delete_tweet(&tweets, 1).await;
    }
}
