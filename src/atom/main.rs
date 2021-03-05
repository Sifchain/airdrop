use anyhow::Result;
use std::env;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};

const PER_PAGE: &str = "100";

#[derive(Deserialize,Debug)]
struct ApiTxsResponse {
    tx: ApiTx,
}

#[derive(Deserialize,Debug)]
struct ApiTx {
   body: ApiTxBody,
}

#[derive(Deserialize,Debug)]
struct ApiTxBody {
    memo: String,
}

#[derive(Deserialize,Debug)]
struct RpcResponse {
    jsonrpc: String,
    id: i32,
    result: RpcResult,
}

type RpcTxs = Vec<RpcTx>;

#[derive(Deserialize,Debug)]
struct RpcResult {
    txs: RpcTxs,
    total_count: String,
}

#[derive(Deserialize,Debug, Clone)]
struct RpcTx {
    pub hash: String,
    height: String,
}

async fn fetch_cosmos_raw_txs(page: u32) -> Result<RpcResponse> {
    let request = format!("https://rpc.cosmos.network/tx_search?query=\"transfer.recipient='{to_address}'\"&per_page={per_page}&page={page}",
                          to_address = "cosmos1ejrf4cur2wy6kfurg9f2jppp2h3afe5h6pkh5t",
                          per_page = PER_PAGE,
                          page = page.to_string()
    );
    println!("request: {}", request);

    let response = reqwest::get(&request)
        .await?
        .json::<RpcResponse>()
        .await?;
    Ok(response)
}

async fn fetch_cosmos_account_txs(address: &str) -> Result<ApiTxsResponse> {
    let request = format!("https://api.cosmostation.io/v1/account/txs/{to_address}", to_address = address);
    println!("request: {}", request);

    let response = reqwest::get(&request)
        .await?
        .json::<ApiTxsResponse>() // TODO change struct if using this function
        .await?;
    Ok(response)
}

async fn fetch_cosmos_txs_details(hash: &str) -> Result<ApiTxsResponse> {
    let request = format!("https://api.cosmos.network/cosmos/tx/v1beta1/txs/{hash}", hash = hash);
    println!("request: {}", request);

    let response = reqwest::get(&request)
        .await?
        .json::<ApiTxsResponse>()
        .await?;

    Ok(response)
}

//
async fn process_cosmos_raw_txs(response: &RpcResponse, pool: &PgPool) -> Result<()>{
    let txs  = response.result.txs.clone();

    for tx in txs {

        // save network, hash and height to db
        match sqlx::query!(
                    r#"
                        INSERT INTO txs (network, hash, height)
                        VALUES ( $1, $2, $3)
                        RETURNING id
                    "#, "ATOM", tx.hash, tx.height,
                ).fetch_one(pool)
                        .await {
            Ok(_) => {
                match fetch_cosmos_txs_details(&tx.hash).await {
                    Ok(resp) => {

                        // save memo to db
                        match sqlx::query!(
                            r#"
                                UPDATE txs
                                SET memo = $1
                                WHERE hash = $2 AND height = $3 AND network = $4
                                RETURNING id
                            "#, resp.tx.body.memo, tx.hash, tx.height, "ATOM" ,
                        ).fetch_one(pool).await {
                            Ok(_) => println!("record saved"),
                            Err(e) => println!("error: {}", e),
                        }
                    },
                    Err(e) => return Err(e),
                };
            },
            Err(_) => println!("already saved: {}", tx.hash),
        }
    }
    Ok(())
}

async fn connect() -> Result<sqlx::postgres::PgPool>{
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&env::var("DATABASE_URL")?).await?;
    Ok(pool)
}


async fn process_incoming_cosmos_txs(pool: &PgPool) -> Result<()>{
    let mut count = 0u32;

    // Get raw txs data
    loop {
        count += 1;
        println!("count: {}",count);

        let response = fetch_cosmos_raw_txs(count).await?;
        process_cosmos_raw_txs(&response, &pool).await?;

        let total: u32 = response.result.total_count.parse().unwrap();
        println!("total_count: {}", total);

        if count == total / 100 + 1 {
            println!("Finished. Break loop");
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect().await?;

    process_incoming_cosmos_txs(&pool).await?;

    Ok(())
}
