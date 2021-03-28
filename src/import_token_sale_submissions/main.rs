use anyhow::anyhow;
use colored::Colorize;
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt::{Display, Formatter};
use std::io;
use utils::address::process_address;
use utils::db::connect;
use utils::twitter::{find_tweet, get_twitter_token, get_twitter_user, process_twitter_handler};

#[derive(Debug, Deserialize)]
struct Record {
    timestamp: String,
    handle: String,
    address: String,
    code: String,
}

#[derive(Debug)]
struct TweetData {
    tweet_found: bool,
    media_found: bool,
    tweet_id: i64,
    tweets_checked: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = connect().await.unwrap();

    let mut records = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_reader(io::stdin());

    let mut counter: i32 = 0;
    let token = get_twitter_token().await?;

    for result in records.deserialize() {
        let record: Record = result?;
        println!("{}{:?}", "\n\nNext Record:".blue(), record);

        match clean_record(&record, &db).await {
            None => {
                if let Ok(found) = is_record_already_processed(&record, Table::InValid, &db).await {
                    println!("Record already processed: {}", record.handle);
                    continue;
                }
                println!("{}: {:?}", "Invalid Submission Record".red(), record);
                save_invalid_record(&record, &db).await;
            }
            Some(v) => {
                if let Ok(found) = is_record_already_processed(&v, Table::Valid, &db).await {
                    println!("Record already processed: {}", v.handle);
                    continue;
                }

                println!("{}: {:?}", "Valid Submission Record".green(), record);
                match get_twitter_user(&v.handle, &token).await {
                    Ok(TwitterUser) => match find_tweet(&TwitterUser, &token).await {
                        Ok((tweet_found, media_found, tweet_id, tweets_checked)) => {
                            let tweet_data = TweetData {
                                tweet_found,
                                media_found,
                                tweet_id,
                                tweets_checked,
                            };
                            println!("{}: {:?}", "TweetData".green(), tweet_data);
                            save_valid_record_with_twitter_check_data(&v, &tweet_data, &db).await;
                        }
                        Err(e) => {
                            println!("{}: {}", "error000", e.to_string().red());
                            save_valid_record_with_twitter_error_data(&record, e.to_string(), &db)
                                .await;
                        }
                    },
                    Err(e) => {
                        println!("{}: {}", "error001", e.to_string().red());
                        if e.to_string().contains("Rate limit") {
                            panic!("Wait for an hour")
                        }
                        save_valid_record_with_twitter_error_data(&record, e.to_string(), &db)
                            .await;
                    }
                }
            }
        }
        counter += 1;
    }

    println!("counter: {}", counter);
    Ok(())
}

async fn save_invalid_record(record: &Record, db: &PgPool) {
    match sqlx::query!(
        r#"
        INSERT into invalid_token_sale_submissions (timestamp, twitter_handle, sif_address, contribution_code)
        VALUES ($1, $2, $3, $4)
    "#,
        record.timestamp,
        record.handle,
        record.address,
        record.code,
    )
    .execute(db)
    .await
    {
        Ok(v) => {
            println!("Invalid Record saved");
        }
        Err(e) => {
            println!("Error saving invalid record: {}", e.to_string().red());
        }
    };
}
async fn save_valid_record_with_twitter_error_data(
    record: &Record,
    tweet_error: String,
    db: &PgPool,
) {
    match sqlx::query!(r#"
        INSERT into token_sale_submissions (timestamp, twitter_handle, sif_address, contribution_code, twitter_error)
        VALUES ($1, $2, $3, $4, $5)
    "#,
        record.timestamp,
        record.handle,
        record.address,
        record.code,
        tweet_error,
    ).execute(db).await {
        Ok(_) => println!("Record inserted"),
        Err(e) => println!("{}: {}","error004".red(), e.to_string().red()),
    }
}
async fn save_valid_record_with_twitter_check_data(
    record: &Record,
    tweet_data: &TweetData,
    db: &PgPool,
) {
    match sqlx::query!(r#"
        INSERT into token_sale_submissions (timestamp, twitter_handle, sif_address, contribution_code, tweet_found, media_found, tweet_id, tweets_checked)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#,
        record.timestamp,
        record.handle,
        record.address,
        record.code,
        tweet_data.tweet_found,
        tweet_data.media_found,
        tweet_data.tweet_id,
        tweet_data.tweets_checked,
    ).execute(db).await{
        Ok(_) => {
            println!("Record inserted");
        },
        Err(e) => {
            println!("{}:{}","error003: ".red(),e.to_string().red());
        },
    }
}

enum Table {
    Valid,
    InValid,
}

async fn is_record_already_processed(
    record: &Record,
    table: Table,
    db: &PgPool,
) -> anyhow::Result<bool> {
    match table {
        Table::Valid => {
            let rows = sqlx::query!(
                r#"
                    SELECT twitter_handle as handle, sif_address as address, contribution_code as code
                    FROM token_sale_submissions
                    WHERE twitter_handle = $1 OR sif_address = $2 OR contribution_code = $3
                "#,
                record.handle,
                record.address,
                record.code,
            )
            .fetch_all(db)
            .await?;
            if rows.len() > 0 {
                return Ok(true);
            }
            Err(anyhow!("No record found"))
        }
        Table::InValid => {
            let rows = sqlx::query!(
                r#"
                    SELECT twitter_handle as handle, sif_address as address, contribution_code as code
                    FROM invalid_token_sale_submissions
                    WHERE twitter_handle = $1 OR sif_address = $2 OR contribution_code = $3
                "#,
                record.handle,
                record.address,
                record.code,
            )
                .fetch_all(db)
                .await?;
            if rows.len() > 0 {
                return Ok(true);
            }
            Err(anyhow!("No record found"))
        }
    }
}

// process/clean record. Return valid record or None is record is not valid.
async fn clean_record(record: &Record, db: &PgPool) -> Option<Record> {
    // process address data
    if let Some(address) = process_address(&record.address) {
        println!("cleaned address: {}", address);
        // process code data
        if let Some(code) = process_code(&record.code) {
            println!("cleaned code: {}", code);
            if let Some(found) = find_valid_code(&record.code, &db).await {
                println!("found valid code: {}", found);
                // clean twitter handle
                if let Some(handle) = process_twitter_handler(&record.handle) {
                    println!("cleaned handle: {}", handle);
                    return Some(Record {
                        timestamp: (*record.timestamp).parse().unwrap(),
                        handle,
                        address,
                        code,
                    });
                }
            }
        }
    }
    None
}

fn process_code(code: &String) -> Option<String> {
    code.trim().parse().ok()
}

async fn find_valid_code(code: &String, db: &PgPool) -> Option<bool> {
    let code = code.replace(" ", "");
    match sqlx::query!(
        r#"
                        SELECT * from token_sale_contribution_codes
                        WHERE contribution_code = $1
                    "#,
        code,
    )
    .fetch_one(db)
    .await
    {
        Ok(_) => Some(true),
        Err(e) => None,
    }
}

#[tokio::test]
async fn test_find_invalid_code() {
    let db = connect().await.unwrap();
    let code = "blahblah".to_string();
    let result = find_valid_code(&code, &db).await;
    assert_eq!(result, None)
}

#[tokio::test]
async fn test_find_valid_code() {
    let db = connect().await.unwrap();
    let code = "ca1d28a0f2bd265edd00e634f7b2548d1f42b4870f4bfa083dfe2328e7efe6e8".to_string();
    let result = find_valid_code(&code, &db).await;
    assert_eq!(result.unwrap(), true)
}
