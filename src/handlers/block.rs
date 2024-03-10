use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub headers: Headers,
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Headers {
    pub block_hash: String,
    pub nonce: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub seq: i32,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
}
