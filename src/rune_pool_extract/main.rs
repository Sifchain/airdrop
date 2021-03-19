use serde::Deserialize;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::result::Result;
use utils::db::connect;

const RPC_ENDPOINT: &str = "http://18.233.24.123:1317/thorchain";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Running Rune pool stakers extract");
    let db = connect().await?;

    let pools = get_pools().await?;

    for pool in pools {
        let stakers = get_pool_stackers(&pool).await?;

        for staker in stakers {
            match sqlx::query!(
                r#"
                    INSERT INTO rune_stakers (asset, rune_address, asset_address, last_stake, last_unstake, units)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING id
                "#,
                staker.asset,
                staker.rune_address,
                staker.asset_address,
                staker.last_stake.parse::<i64>().unwrap(),
                staker.last_unstake.parse::<i64>().unwrap(),
                staker.units.parse::<i64>().unwrap(),
            )
                .fetch_one(&db)
                .await
            {
                Ok(_) => println!("Record saved"),
                Err(_) => println!("Already saved")
            };
        }
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
