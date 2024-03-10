use chrono::Utc;

use super::block::{Block, Headers, Payload};

pub struct Blockchain {
    pub difficulty: i32,
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(difficulty: i32) -> Blockchain {
        let mut blockchain = Blockchain {
            difficulty,
            chain: Vec::new(),
        };
        blockchain.generate_genesis_block();

        blockchain
    }

    #[allow(dead_code)]
    fn generate_genesis_block(&mut self) {
        let payload = Payload {
            seq: 0,
            data: String::from("Genesis Block"),
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from(""),
        };
        let block = Block {
            headers: Headers {
                block_hash: "123".to_string(),
                nonce: 0,
            },
            payload,
        };

        self.chain.insert(0, block)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_blockchain() {
        let blockchain = Blockchain::new(4);
        assert_eq!(blockchain.chain[0].payload.data, "Genesis Block");
        assert_eq!(blockchain.chain[0].payload.seq, 0)
    }
}
