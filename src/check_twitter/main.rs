use anyhow::anyhow;
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
            DISTINCT ON (twitter_handle) 
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
        let handle = match record.twitter_handle {
            Some(v) => {
                println!("\n\nhandle: {}", v);

                match get_twitter_user(v, &token).await {
                    Ok(v) => {
                        match find_tweet(&v, &token).await {
                            Ok((tweet_found, media_found, id)) => {
                                println!("{}", "Found Required tweet".green());
                                // let id = id as i64;
                                sqlx::query!(
                                    r#"
                                        UPDATE txs
                                        SET tweet_found = $5, tweet_id = $4, media_found = $6
                                        WHERE id = $1 AND hash = $2 AND height = $3
                                        RETURNING id
                                    "#,
                                    record.id,
                                    record.hash,
                                    record.height,
                                    id,
                                    tweet_found,
                                    media_found,
                                )
                                .fetch_one(&db)
                                .await;
                            }
                            Err(e) => {
                                println!("error001: {}", e.to_string().red());

                                sqlx::query!(
                                    r#"
                                    UPDATE txs
                                    SET twitter_error = $4, tweet_found = false, media_found = false
                                    WHERE id = $1 AND hash = $2 AND height = $3
                                "#,
                                    record.id,
                                    record.hash,
                                    record.height,
                                    format!("{}", e),
                                )
                                .fetch_one(&db)
                                .await;
                            }
                            _ => {
                                println!("{}", "No match pattern".red());
                            }
                        };
                    }
                    // Error like user has been deleted.
                    Err(e) => {
                        println!("error002: {}", e.to_string().red());

                        if e.to_string().contains("Rate limit") {
                            panic!("Wait an hour for next run")
                        }

                        sqlx::query!(
                            r#"
                                    UPDATE txs
                                    SET twitter_error = $4, tweet_found = false, media_found = false
                                    WHERE id = $1 AND hash = $2 AND height = $3
                                "#,
                            record.id,
                            record.hash,
                            record.height,
                            format!("{}", e),
                        )
                        .fetch_one(&db)
                        .await;
                    }
                };
            }
            None => println!("No handle"),
        };
    }
    Ok(())
}

// Return (tweet_found, media_found, tweet_id)
async fn find_tweet(
    user: &egg_mode::user::TwitterUser,
    token: &egg_mode::Token,
) -> anyhow::Result<(bool, bool, i64)> {
    let timeline = egg_mode::tweet::user_timeline(user.id, true, false, token).with_page_size(200);
    let mut tweets_checked: u32 = 0;

    let (timeline, feed) = timeline.start().await?;
    for tweet in feed.iter() {
        let (tweet_found, media_found) = check_tweet(tweet).await?;
        tweets_checked += 1;

        if tweet_found && media_found {
            println!("tweets_checked: {}", tweets_checked);
            let id = tweet.id as i64;
            return Ok((tweet_found, media_found, id));
        }
    }

    let (timeline, feed) = timeline.older(None).await?;
    println!("checking older tweets...");
    for tweet in feed.iter() {
        let (tweet_found, media_found) = check_tweet(tweet).await?;
        tweets_checked += 1;

        if tweet_found && media_found {
            println!("tweets_checked: {}", tweets_checked);
            let id = tweet.id as i64;
            return Ok((tweet_found, media_found, id));
        }
    }

    let (timeline, feed) = timeline.older(None).await?;
    println!("checking more older tweets...");
    for tweet in feed.iter() {
        let (tweet_found, media_found) = check_tweet(tweet).await?;
        tweets_checked += 1;

        if tweet_found && media_found {
            println!("tweets_checked: {}", tweets_checked);
            let id = tweet.id as i64;
            return Ok((tweet_found, media_found, id));
        }
    }

    println!("tweets_checked: {}", tweets_checked);
    Err(anyhow!(
        "tweet not found, checked {} tweets",
        tweets_checked
    ))
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
