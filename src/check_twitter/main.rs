use anyhow::Error;
use colored::Colorize;
use egg_mode;
use egg_mode::tweet::ExtendedTweetEntities;
use egg_mode::user::TwitterUser;
use std::env;
use utils::db::connect;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = connect().await.unwrap();

    let records = sqlx::query!(
        r#"
        SELECT      
            *
        FROM txs 
        WHERE tweet_found is null 
    "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    println!("Total Unique Tweeter Accounts: {}", records.len());

    let token = get_twitter_token().await.unwrap();

    for record in records {
        println!("\n\nTwitter handle: {:?}", &record.twitter_handle);
        let handle = match record.twitter_handle {
            Some(v) => {
                match get_twitter_user(v, &token).await {
                    Ok(v) => {
                        match find_tweet(&v, &token).await {
                            Ok((tweet_found, media_found, id, tweets_checked)) => {
                                println!("tweet_found: {}", tweet_found);
                                println!("media_found: {}", media_found);
                                println!("tweet_id: {}", id);
                                println!("tweets_checked: {}", tweets_checked);

                                match sqlx::query!(
                                    r#"
                                        UPDATE txs
                                        SET tweet_found = $5, tweet_id = $4, media_found = $6, tweets_checked = $7
                                        WHERE id = $1 AND hash = $2 AND height = $3
                                        RETURNING id
                                    "#,
                                    record.id,
                                    record.hash,
                                    record.height,
                                    id,
                                    tweet_found,
                                    media_found,
                                    tweets_checked,
                                )
                                .fetch_one(&db)
                                .await
                                {
                                    Ok(v) => println!("Record updated"),
                                    Err(e) => println!("Record update error: {}", e),
                                }
                            }
                            Err(e) => {
                                println!("error000: {}", e.to_string().red());
                                match sqlx::query!(
                                    r#"
                                    UPDATE txs 
                                    SET twitter_error = $4, tweet_found = false, media_found = false
                                    WHERE id = $1 AND hash = $2 AND height = $3
                                    RETURNING id
                                "#,
                                    record.id,
                                    record.hash,
                                    record.height,
                                    "No match pattern",
                                )
                                .fetch_one(&db)
                                .await
                                {
                                    Ok(v) => println!("Record updated"),
                                    Err(e) => println!("Record update error: {}", e),
                                }
                            }
                        };
                    }
                    // Error like user has been deleted.
                    Err(e) => {
                        println!("error001: {}", e.to_string().red());

                        if e.to_string().contains("Rate limit") {
                            panic!("Wait an hour for next run")
                        }

                        match sqlx::query!(
                            r#"
                                    UPDATE txs
                                    SET twitter_error = $4, tweet_found = false, media_found = false
                                    WHERE id = $1 AND hash = $2 AND height = $3
                                    RETURNING id
                                "#,
                            record.id,
                            record.hash,
                            record.height,
                            format!("{}", e),
                        )
                        .fetch_one(&db)
                        .await
                        {
                            Ok(v) => println!("Record updated"),
                            Err(e) => println!("Record update error: {}", e),
                        }
                    }
                };
            }
            None => {
                println!("{}", "No handle".red());
                match sqlx::query!(
                    r#"
                    UPDATE txs 
                    SET tweet_found = false, media_found = false, twitter_error = 'invalid handle'
                    WHERE id = $1 AND hash = $2 AND height = $3
                    RETURNING id
                "#,
                    record.id,
                    record.hash,
                    record.height,
                )
                .fetch_one(&db)
                .await
                {
                    Ok(v) => println!("Record updated"),
                    Err(e) => println!("Record update error: {}", e),
                }
            }
        };
        break;
    }
    Ok(())
}

// Return (tweet_found, media_found, tweet_id)
async fn find_tweet(
    user: &egg_mode::user::TwitterUser,
    token: &egg_mode::Token,
) -> anyhow::Result<(bool, bool, i64, i32)> {
    let timeline = egg_mode::tweet::user_timeline(user.id, true, false, token).with_page_size(200);

    let mut tweet_found: bool = false;
    let mut media_found: bool = false;
    let mut tweet_id: i64 = -1;
    let mut tweets_checked: i32 = 0;

    let (timeline, feed) = timeline.start().await?;
    for tweet in feed.iter() {
        let (tfound, mfound) = check_tweet(tweet).await?;
        tweets_checked += 1;

        if tfound == true {
            tweet_found = tfound;
            tweet_id = tweet.id as i64;
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

    let (timeline, feed) = timeline.older(None).await?;
    println!("checking older tweets...");
    for tweet in feed.iter() {
        let (tfound, mfound) = check_tweet(tweet).await?;
        tweets_checked += 1;

        if tfound == true {
            tweet_found = tfound;
            tweet_id = tweet.id as i64;
        }

        if mfound == true {
            media_found = mfound;
        }

        if tweet_found && media_found {
            let id = tweet.id as i64;
            return Ok((tweet_found, media_found, id, tweets_checked));
        }
    }

    let (timeline, feed) = timeline.older(None).await?;
    println!("checking more older tweets...");
    for tweet in feed.iter() {
        let (tfound, mfound) = check_tweet(tweet).await?;
        tweets_checked += 1;
        if tfound == true {
            tweet_found = tfound;
            tweet_id = tweet.id as i64;
        }

        if mfound == true {
            media_found = mfound;
        }

        if tweet_found && media_found {
            let id = tweet.id as i64;
            return Ok((tweet_found, media_found, id, tweets_checked));
        }
    }
    Ok((tweet_found, media_found, tweet_id, tweets_checked))
}

// Return (tweet_found, media_found)
async fn check_tweet(tweet: &egg_mode::tweet::Tweet) -> anyhow::Result<(bool, bool)> {
    match tweet.text.to_lowercase().find("sifchain") {
        None => Ok((false, false)),
        Some(v) => {
            println!("Tweet found: {}", tweet.text);
            match &tweet.extended_entities {
                None => Ok((true, false)),
                Some(v) => {
                    println!("Media found: true");
                    Ok((true, true))
                }
            }
        }
    }
}

async fn get_twitter_user(
    handle: String,
    token: &egg_mode::Token,
) -> anyhow::Result<egg_mode::user::TwitterUser> {
    Ok(egg_mode::user::show(handle, &token).await?.response)
}

async fn get_twitter_user_id(handle: String, token: &egg_mode::Token) -> anyhow::Result<u64> {
    Ok(egg_mode::user::show(handle, &token).await?.response.id)
}

async fn get_twitter_token() -> anyhow::Result<egg_mode::Token> {
    let consumer_key = env::var("CONSUMER_KEY")?;
    let consumer_secret = env::var("CONSUMER_SECRET")?;
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    Ok(egg_mode::auth::bearer_token(&con_token).await?)
}

#[tokio::test]
async fn find_tweet_test001() {
    let token = get_twitter_token().await.unwrap();
    let user = get_twitter_user("mihailborodatyi".to_string(), &token)
        .await
        .unwrap();
    let (tweet_found, media_found, tweet_id, tweets_checked) =
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
    let (tweet_found, media_found, tweet_id, tweets_checked) =
        find_tweet(&user, &token).await.unwrap();

    assert_eq!(tweet_found, true);
    assert_eq!(media_found, true);
}
