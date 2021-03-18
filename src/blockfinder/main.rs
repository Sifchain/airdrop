use serde::{de, Deserialize, Deserializer};
use chrono::{NaiveDateTime, NaiveDate};

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

const COMP_END_DATE_UTC: &str = "2021-02-26 06:00:00";

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let end_comp_datetime_utc = NaiveDateTime::parse_from_str(COMP_END_DATE_UTC,"%Y-%m-%d %H:%M:%S").unwrap();
    println!("end_comp_datetime_utc: {:?}", end_comp_datetime_utc);

    let cosmos_start_height = 5200791;
    let mut cosmos_block_height = 5450741;

    loop {
        println!("block: {}", cosmos_block_height);
        let request = format!("https://rpc.cosmos.network/block?height={height}", height = cosmos_block_height);
        println!("request: {:?}", request);
        let cosmos_response = reqwest::get(&request)
            .await?
            .json::<RpcResponse>()
            .await?;

        let response_datetime = cosmos_response.result.block.header.time;
        let duration = end_comp_datetime_utc.signed_duration_since(response_datetime);

        println!("response datetime: {:?}", response_datetime);
        println!("duration {:?}", duration.num_hours());

        if duration.num_hours() == 1 {
            println!("Found block height within an hour: {}", cosmos_block_height);
            break;
        } else {
            if duration.num_hours() > 0 {
                let x = (cosmos_block_height - cosmos_start_height) * 2;
                println!("x: {:?}", x);
                cosmos_block_height = cosmos_block_height - x;
            }

            if duration.num_hours() < 0 {
                let x = (cosmos_block_height - cosmos_start_height) / 2;
                println!("x: {:?}", x);
                cosmos_block_height = cosmos_block_height - x;
            }
        }
    }

    //
    // let bnb_response = reqwest::get("https://dataseed1.binance.org/block?height=1")
    //     .await?
    //     .json::<RpcResponse>()
    //     .await?;
    //
    // println!("bnb response: {:?}", bnb_response);

    Ok(())
}
