pub async fn find_tweet(handle: String) -> anyhow::Result<bool> {
    println!("handle: {}", handle);
    Ok(true)
}

mod test {
    use super::*;

    #[tokio::test]
    // No tweet
    async fn find_tweet_test00() {
        let handle = "utx0".to_string();
        let result = find_tweet(handle).await;
        assert_eq!(result.unwrap(), false);
    }
}
