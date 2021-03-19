use anyhow::Result;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use utils::memo::process_memo;

const PER_PAGE: &str = "50";

#[derive(Deserialize, Debug)]
struct ApiResponse {
    txNums: u32,
    txArray: Vec<Tx>,
}

#[derive(Deserialize, Debug, Clone)]
struct Tx {
    blockHeight: u32,
    code: u32,
    txHash: String,
    txType: String,
    txAsset: String,
    value: f64,
    txFee: f64,
    fromAddr: String, // TODO use this.
    toAddr: String,
    txAge: u32,
    log: String,
    confirmBlocks: u32,
    memo: String,
    source: u32,
    timeStamp: u64,
}

async fn fetch_rune_raw_txs(page: u32) -> Result<ApiResponse> {
    let request = format!("https://api-binance-mainnet.cosmostation.io/v1/account/txs/{to_address}?page={page}&rows={per_page}",
                          to_address = "bnb1p0kmzzrq0220agpey43nyp826pgh9rc5y9a4m3",
                          per_page = PER_PAGE,
                          page = page.to_string()
    );
    println!("request: {}", request);
    let response = reqwest::get(&request).await?.json::<ApiResponse>().await?;
    Ok(response)
}

//
async fn process_rune_raw_txs(response: &ApiResponse, pool: &PgPool) -> Result<()> {
    for tx in response.txArray.clone() {
        let memo = process_memo(&tx.memo);
        match sqlx::query!(
            r#"
                        INSERT INTO txs (network, hash, height, memo, twitter_handle, sif_address)
                        VALUES ( $1, $2, $3, $4, $5, $6)
                        RETURNING id
                    "#,
            "RUNE",
            tx.txHash,
            tx.blockHeight.to_string(),
            tx.memo,
            memo.handle,
            memo.address
        )
        .fetch_one(pool)
        .await
        {
            Ok(_) => println!("record saved"),
            Err(_) => println!("Already saved"),
        };
    }
    Ok(())
}

async fn connect() -> Result<sqlx::postgres::PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    Ok(pool)
}

async fn process_incoming_rune_txs(pool: &PgPool) -> Result<()> {
    let mut count = 0u32;

    // Get raw txs data
    loop {
        count += 1;
        println!("count: {}", count);

        let response = fetch_rune_raw_txs(count).await?;
        process_rune_raw_txs(&response, &pool).await?;

        let total: u32 = response.txNums;
        println!("total_count: {}", total);

        if count == total / PER_PAGE.parse::<u32>().unwrap() + 1 {
            println!("Finished. Break loop");
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect().await?;

    process_incoming_rune_txs(&pool).await?;

    Ok(())
}
