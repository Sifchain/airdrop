use serde::Deserialize;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::result::Result;

const RPC_ENDPOINT: &str = "http://18.233.24.123:1317/thorchain";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Running Rune pool stakers extract");

    let pools = get_pools().await?;

    for pool in pools {
        let stakers = get_pool_stackers(&pool).await?;
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Pool {
    balance_rune: String,
    balance_asset: String,
    asset: String,
    status: String,
}

async fn get_pools() -> anyhow::Result<Vec<Pool>> {
    let request = format!("{}/pools", RPC_ENDPOINT);
    println!("request: {}", request);
    Ok(reqwest::get(&request).await?.json::<Vec<Pool>>().await?)
}

#[derive(Deserialize, Debug)]
struct Staker {
    asset: String,
    rune_address: String,
    asset_address: String,
    last_stake: String,
    last_unstake: String,
    units: String,
    pending_rune: String,
    pending_tx_id: String,
}

async fn get_pool_stackers(pool: &Pool) -> anyhow::Result<Vec<Staker>> {
    let request = format!("{}/pool/{pool}/stakers", RPC_ENDPOINT, pool = pool.asset);
    println!("request: {}", request);
    Ok(reqwest::get(&request).await?.json::<Vec<Staker>>().await?)
}
