use reqwest::Result;
use reqwest::Error;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

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
    hash: String,
    height: String,
}

async fn fetch_cosmos_raw_txs(page: u32) -> Result<(RpcResponse)> {
    let request = format!("https://rpc.cosmos.network/tx_search?query=\"transfer.recipient='{to_address}'\"&per_page={per_page}&page={page}",
                          to_address = "cosmos1ejrf4cur2wy6kfurg9f2jppp2h3afe5h6pkh5t",
                          per_page = "100",
                          page = page.to_string()
    );
    println!("request: {}", request);

    let response = reqwest::get(&request)
        .await?
        .json::<RpcResponse>()
        .await?;
    Ok(response)
}

async fn fetch_cosmos_account_txs(address: &str) -> Result<(ApiTxsResponse)> {
    let request = format!("https://api.cosmostation.io/v1/account/txs/{to_address}", to_address = address);
    println!("request: {}", request);

    let response = reqwest::get(&request)
        .await?
        .json::<ApiTxsResponse>() // TODO change struct if using this function
        .await?;
    Ok(response)
}

async fn fetch_cosmos_txs_details(hash: &str) -> Result<(ApiTxsResponse)> {
    let request = format!("https://api.cosmos.network/cosmos/tx/v1beta1/txs/{hash}", hash = hash);
    println!("request: {}", request);

    let response = reqwest::get(&request)
        .await?
        .json::<ApiTxsResponse>()
        .await?;

    Ok(response)
}

fn save_rpc_response_to_db(response: &RpcResponse) {
    // println!("{:#?}", response);

    let txs  = response.result.txs.clone();

    for tx in txs {
        println!("{:?}", tx);
    }
}

async fn connect() -> sqlx::Result<(sqlx::Pool<Postgres>)> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/txs").await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect().await?;

    let mut count = 0u32;

    // Get raw txs data
    loop {
        count += 1;
        println!("count: {}",count);

        let response = fetch_cosmos_raw_txs(count).await?;
        // println!("{:#?}", response);
        save_rpc_response_to_db(&response);


        let mut total: u32 = response.result.total_count.parse().unwrap();
        println!("total_count: {}", total);

        if count == total / 100 + 1 {
            println!("Break loop");
            break;
        }
    }

    // let response = fetch_cosmos_txs_details("D5F3927B9BCE4F429155B60E626FCDDBC39FC078C435B038BAE358D51FB1A494").await?;
    // println!("{:#?}", response);

    Ok(())
}
