use crate::repositories::Tweets;
use crate::views::Home;

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list().await;
    Home {
        tweets: tweets.into_iter().map(|x| x.into()).collect(),
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
}
