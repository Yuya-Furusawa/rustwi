use crate::entities::Tweet;
use crate::repositories::Tweets;
use crate::views::Home;

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list().await;
    Home {
        tweets: tweets.into_iter().map(|x| x.into()).collect(),
    }
}

pub async fn create_tweet(repo: &impl Tweets, message: &str) {
    let new_tweet = Tweet::create(message);
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

    use crate::entities::Tweet;
    use crate::repositories::MockTweets;

    fn tweet(id: i32) -> Tweet {
        Tweet::new(
            id,
            format!("message{}", id),
            Utc.ymd(2020, 1, 1).and_hms(0, 0, 0),
        )
    }

    #[tokio::test]
    async fn test_list_tweets() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![tweet(2), tweet(1)]);

        let result = super::list_tweets(&tweets).await;
        assert_eq!(result.tweets.len(), 2);
        let result0 = result.tweets.get(0).unwrap();
        assert_eq!(result0.message, "message2");
        assert_eq!(result0.posted_at, "2020/01/01 00:00");
    }

    #[tokio::test]
    async fn test_list_tweets_empty() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![]);

        let result = super::list_tweets(&tweets).await;
        assert_eq!(result.tweets.is_empty(), true);
    }

    #[tokio::test]
    async fn test_create_tweet() {
        let mut tweets = MockTweets::new();
        tweets
            .expect_store()
            .withf(|e| e.message == tweet(1).message)
            .once()
            .return_const(());

        let tweet = tweet(1);
        super::create_tweet(&tweets, &tweet.message).await;
    }

    #[tokio::test]
    async fn test_delete_tweet() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| Some(tweet(1)));
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
