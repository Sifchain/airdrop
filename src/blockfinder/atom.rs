use serde::Deserialize;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Deserialize,Debug)]
struct rpcresponse {
    jsonrpc: String,
    id: i32,
    result: Block,
}

#[derive(Deserialize,Debug)]
struct Block {
    header: Header,
}

#[derive(Deserialize,Debug)]
struct Header {
    height: u32,
    time: DateTime<Utc>,
    test: NaiveDateTime
}
