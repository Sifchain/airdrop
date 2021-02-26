use reqwest::Error;
use serde::Deserialize;

type Txs = Vec<Tx>;

#[derive(Deserialize,Debug)]
struct ApiResponse {
    jsonrpc: String,
    id: i32,
    result: Res,
    // total_count: u32,
}

#[derive(Deserialize,Debug)]
struct Res {
    txs: Txs,
    total_count: String,
}

#[derive(Deserialize,Debug)]
struct Tx{
    hash: String,
    height: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let request_url = format!("https://api.cosmostation.io/v1/account/txs/{to_address}",
    //                           to_address = "cosmos1ejrf4cur2wy6kfurg9f2jppp2h3afe5h6pkh5t");
    //
    // println!("{}", request_url);
    // let response = reqwest::get(&request_url).await?;
    //
    // // println!("{:?}", response.text().await?);
    //
    // let data: Vec<Payload> = response.json().await?;
    // println!("{}",data.len());
    // // println!("{:#?}", data);
    // Ok(())


    let reqwest_url = format!("https://rpc.cosmos.network/tx_search?query=\"transfer.recipient='{to_address}'\"&per_page={per_page}&page={page}",
                                to_address = "cosmos1ejrf4cur2wy6kfurg9f2jppp2h3afe5h6pkh5t",
                                per_page = "100",
                                page = "1"
    );

    // println!("{}", reqwest_url);
    //
    // let response = reqwest::get(&reqwest_url).await?;
    //
    // println!("{}", response.text().await?);

    let response = reqwest::get(&reqwest_url)
        .await?
        .json::<ApiResponse>()
        .await?;

    println!("{:#?}", response);

    println!("{}", response.result.txs.len());

    Ok(())
}
