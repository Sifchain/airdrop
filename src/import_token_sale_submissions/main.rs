use colored::Colorize;
use serde::Deserialize;
use sqlx::PgPool;
use std::io;
use utils::address::process_address;
use utils::db::connect;
use utils::twitter::process_twitter_handler;

#[derive(Debug, Deserialize)]
struct Record {
    timestamp: String,
    handle: String,
    address: String,
    code: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = connect().await.unwrap();

    let mut records = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_reader(io::stdin());

    let mut counter: i32 = 0;

    for result in records.deserialize() {
        let record: Record = result?;

        match process_record(&record, &db).await {
            None => {
                save_invalid_record(&record, &db).await;
            }
            Some(v) => {
                save_valid_record(&v, &db).await;
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
        Ok(v) => {}
        Err(e) => {}
    };
}

async fn save_valid_record(record: &Record, db: &PgPool) {
    match sqlx::query!(r#"
        INSERT into token_sale_submissions (timestamp, twitter_handle, sif_address, contribution_code)
        VALUES ($1, $2, $3, $4)
    "#,
        record.timestamp,
        record.handle,
        record.address,
        record.code,
    ).execute(db).await{
        Ok(v) => {},
        Err(e) => {},
    }
}

// process/clean record. Return valid record or None is record is not valid.
async fn process_record(record: &Record, db: &PgPool) -> Option<Record> {
    // process address data
    if let Some(address) = process_address(&record.address) {
        // process code data
        if let Some(code) = process_code(&record.code) {
            if let Some(_) = find_valid_code(&record.code, &db).await {
                // clean twitter handle
                if let Some(handle) = process_twitter_handler(&record.handle) {
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
        Err(e) => Some(false),
    }
}
