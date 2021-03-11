use serde::{de, Deserialize, Deserializer};
use chrono::{NaiveDateTime};

#[derive(Deserialize,Debug)]
pub struct RpcResponse {
    jsonrpc: String,
    result: Resultz,
}

#[derive(Deserialize,Debug)]
struct Resultz {
   block: Block
}

#[derive(Deserialize,Debug)]
struct Block {
    header: Header,
}

#[derive(Deserialize,Debug)]
pub struct Header {
    height: String,
    chain_id: String,
    #[serde(deserialize_with = "atom_naive_date_time_from_str")]
    time: NaiveDateTime,
}

fn atom_naive_date_time_from_str<'de, D> (deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ") {
        Ok(res) => Ok(res),
        Err(_) => {
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ").map_err(de::Error::custom)
        }
    }
}

mod tests {
    use super::*;

    #[test] // cosmos hub
    fn deserialize01(){

        let result: Header = serde_json::from_str(
            r#"{ "chain_id": "1", "height": "1","time": "2019-12-11T16:11:34Z"}"#,
        ).unwrap();

        assert_eq!(result.height, "1");
        assert_eq!(result.time.to_string(), "2019-12-11 16:11:34");
    }

    #[test] // bnb chain
    fn deserialize02(){
        let result: Header = serde_json::from_str(
            r#"{ "chain_id":"1", "height": "1","time": "2019-04-18T05:59:26.228734998Z"}"#,
        ).unwrap();
        assert_eq!(result.height, "1");
        assert_eq!(result.time.to_string(), "2019-04-18 05:59:26.228734998");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let cosmos_response = reqwest::get("https://rpc.cosmos.network/block?height=5200791")
        .await?
        .json::<RpcResponse>()
        .await?;

    println!("cosmos response: {:?}", cosmos_response);

    let bnb_response = reqwest::get("https://dataseed1.binance.org/block?height=1")
        .await?
        .json::<RpcResponse>()
        .await?;

    println!("bnb response: {:?}", bnb_response);

    Ok(())
}
