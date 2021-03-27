use colored::Colorize;
use egg_mode;
use std::env;
use utils::db::connect;
use utils::twitter::*;

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
        match record.twitter_handle {
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
                                    Ok(_) => println!("Record updated"),
                                    Err(e) => println!("Record update error: {}", e),
                                }
                            }
                            Err(e) => {
                                println!("error000: {}", e.to_string().red());

                                if e.to_string().contains("Rate limit reached") {
                                    panic!("wait an hour")
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
                                    Ok(_) => println!("Record updated"),
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
                            Ok(_) => println!("Record updated"),
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
                    Ok(_) => println!("Record updated"),
                    Err(e) => println!("Record update error: {}", e),
                }
            }
        };
    }
    Ok(())
}
