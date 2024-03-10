#[derive(Debug, Clone)]
pub struct Block {
    pub headers: Headers,
    pub payload: Payload,
}

#[derive(Debug, Clone)]
pub struct Headers {
    pub block_hash: String,
    pub nonce: i32,
}

#[derive(Debug, Clone)]
pub struct Payload {
    pub seq: i32,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
}
