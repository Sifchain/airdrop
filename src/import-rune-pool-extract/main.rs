use serde::Deserialize;
use utils::db::connect;

const RPC_ENDPOINT: &str = "http://18.233.24.123:1317/thorchain";
const API_ENDPOINT: &str = "https://chaosnet-midgard.bepswap.com/v1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Running Rune pool stakers extract");
    let db = connect().await?;

    let pools = get_pools().await?;

    for pool in pools {
        let mpool = get_mpool(&pool).await?;
        let pool_price = mpool[0].price.parse::<f64>().unwrap();
        let pool_units = mpool[0].poolUnits.parse::<i64>().unwrap();
        let asset_depth = mpool[0].assetDepth.parse::<i64>().unwrap();
        let rune_depth = mpool[0].runeDepth.parse::<i64>().unwrap();
        let stakers = get_pool_stackers(&pool).await?;

        for staker in stakers {
            let staker_units = staker.units.parse::<i64>().unwrap();
            let staker_share = staker_units as f64 / pool_units as f64;
            let staker_rune = staker_share * rune_depth as f64;
            let staker_asset = staker_share * asset_depth as f64;
            let asset_total_rune = staker_asset * pool_price;
            let staker_total_rune = asset_total_rune as i64 + staker_rune as i64;

            match sqlx::query!(
                r#"
                    INSERT INTO rune_stakers (asset, rune_address, asset_address, last_stake, last_unstake, units, stake_total_rune)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    RETURNING id
                "#,
                staker.asset,
                staker.rune_address,
                staker.asset_address,
                staker.last_stake.parse::<i64>().unwrap(),
                staker.last_unstake.parse::<i64>().unwrap(),
                staker.units.parse::<i64>().unwrap(),
                staker_total_rune,
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
#[derive(Deserialize, Debug)]
struct MPool {
    asset: String,
    assetDepth: String,
    assetEarned: String,
    assetStakedTotal: String,
    buyAssetCount: String,
    buyFeeAverage: String,
    buyFeesTotal: String,
    buySlipAverage: String,
    buyTxAverage: String,
    buyVolume: String,
    poolAPY: String,
    poolDepth: String,
    poolEarned: String,
    poolFeeAverage: String,
    poolFeesTotal: String,
    poolSlipAverage: String,
    poolStakedTotal: String,
    poolTxAverage: String,
    poolUnits: String,
    poolVolume: String,
    poolVolume24hr: String,
    price: String,
    runeDepth: String,
    runeEarned: String,
    runeStakedTotal: String,
    sellAssetCount: String,
    sellFeeAverage: String,
    sellFeesTotal: String,
    sellSlipAverage: String,
    sellTxAverage: String,
    sellVolume: String,
    stakeTxCount: String,
    stakersCount: String,
    stakingTxCount: String,
    status: String,
    swappersCount: String,
    swappingTxCount: String,
    withdrawTxCount: String,
}


async fn get_pools() -> anyhow::Result<Vec<Pool>> {
    let request = format!("{}/pools", RPC_ENDPOINT);
    println!("request: {}", request);
    Ok(reqwest::get(&request).await?.json::<Vec<Pool>>().await?)
}

async fn get_mpool(pool: &Pool) -> anyhow::Result<Vec<MPool>> {
    let request = format!("{}/pools/detail?asset={pool}", API_ENDPOINT, pool=pool.asset);
    println!("request: {}", request);
    Ok(reqwest::get(&request).await?.json::<Vec<MPool>>().await?)
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
