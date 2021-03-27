use std::env;

pub fn process_twitter_handler(twitter_extract: &String) -> Option<String> {
    Some(twitter_extract.replace("@", "").trim().parse().unwrap())
}

pub async fn get_twitter_user(
    handle: &String,
    token: &egg_mode::Token,
) -> anyhow::Result<egg_mode::user::TwitterUser> {
    println!("Handle: {}", handle);
    Ok(egg_mode::user::show(handle.to_string(), &token)
        .await?
        .response)
}

pub async fn get_twitter_token() -> anyhow::Result<egg_mode::Token> {
    let consumer_key = env::var("CONSUMER_KEY")?;
    let consumer_secret = env::var("CONSUMER_SECRET")?;
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    Ok(egg_mode::auth::bearer_token(&con_token).await?)
}

pub async fn find_tweet(
    user: &egg_mode::user::TwitterUser,
    token: &egg_mode::Token,
) -> anyhow::Result<(bool, bool, i64, i32)> {
    let timeline = egg_mode::tweet::user_timeline(user.id, true, false, token).with_page_size(200);

    let mut tweet_found: bool = false;
    let mut media_found: bool = false;
    let mut tweets_checked: i32 = 0;

    let (mut timeline, mut feed) = timeline.start().await?;
    let mut id: u64 = 0;
    // let mut last_id: u64 = 0;

    let mut fetch_count: i32 = 0;

    loop {
        for tweet in feed.iter() {
            let (tfound, mfound) = check_tweet(tweet).await?;
            tweets_checked += 1;

            id = tweet.id;

            if tfound == true {
                tweet_found = tfound;
            }

            if mfound == true {
                media_found = mfound;
            }

            // Return if both are found.
            if tweet_found && media_found {
                let id = tweet.id as i64;
                return Ok((tweet_found, media_found, id, tweets_checked));
            }
        }
        println!("Check next lot of tweets. id: {}", id);
        // This gets stuck on the same tweet for some unknown reason
        // if last_id == id {
        //     return Ok((tweet_found, media_found, -1, tweets_checked));
        // }
        // last_id = id;

        let (ntimeline, nfeed) = timeline.older(Some(id)).await?;
        timeline = ntimeline;
        feed = nfeed;
        fetch_count += 1;

        // due to some random behavior of the twitter api or the egg_mode crate this is required.
        if fetch_count == 20 {
            return Ok((tweet_found, media_found, -1, tweets_checked));
        }
    }
}

// Return (tweet_found, media_found)
async fn check_tweet(tweet: &egg_mode::tweet::Tweet) -> anyhow::Result<(bool, bool)> {
    match tweet.text.to_lowercase().find("sifchain") {
        None => Ok((false, false)),
        Some(_) => {
            // println!("Tweet found: {}", tweet.text);
            match &tweet.extended_entities {
                None => {
                    if tweet.entities.urls.len() > 0 {
                        println!("Media url: {:#?}", tweet.entities.urls.len());
                        return Ok((true, true));
                    }
                    Ok((true, false))
                }
                Some(_) => {
                    // println!("Media found: true");
                    Ok((true, true))
                }
            }
        }
    }
}

#[tokio::test]
async fn find_tweet_test001() {
    let token = get_twitter_token().await.unwrap();
    let user = get_twitter_user("mihailborodatyi".to_string(), &token)
        .await
        .unwrap();
    let (tweet_found, media_found, _tweet_id, _tweets_checked) =
        find_tweet(&user, &token).await.unwrap();

    assert_eq!(tweet_found, true);
    assert_eq!(media_found, false);
}

// test for a url media link
#[tokio::test]
async fn find_tweet_test002() {
    let token = get_twitter_token().await.unwrap();
    let user = get_twitter_user("kastrosphere".to_string(), &token)
        .await
        .unwrap();
    let (tweet_found, media_found, _tweet_id, _tweets_checked) =
        find_tweet(&user, &token).await.unwrap();

    assert_eq!(tweet_found, true);
    assert_eq!(media_found, true);
}
