use chrono::Utc;

use crate::infra::hasher::generate_hash;

use super::block::{Block, Headers, Payload};

pub struct Blockchain {
    pub difficulty: i32,
    pub chain: Vec<Block>,
}

#[allow(dead_code)]
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
        let serialized_payload = serde_json::to_string(&payload).unwrap();
        let block = Block {
            headers: Headers {
                block_hash: generate_hash(&serialized_payload),
                nonce: 0,
            },
            payload,
        };

        self.chain.insert(0, block)
    }

    fn last_block(&mut self) -> Block {
        let block = match self.chain.last() {
            Some(block) => block,
            _ => panic!("Last block not found"),
        };
        block.to_owned()
    }

    fn last_hash(&mut self) -> String {
        let block = self.last_block();
        block.headers.block_hash
    }

    pub fn create_block(&mut self, data: String) -> Payload {
        let last_block = self.last_block();
        let payload = Payload {
            seq: last_block.payload.seq + 1,
            previous_hash: self.last_hash(),
            data,
            timestamp: Utc::now().timestamp(),
        };
        payload
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_blockchain() {
        let blockchain = Blockchain::new(4);
        assert_eq!(blockchain.chain[0].payload.data, "Genesis Block");
        assert_eq!(blockchain.chain[0].payload.seq, 0);
        assert_ne!(blockchain.chain[0].headers.block_hash, "")
    }

    #[test]
    fn should_return_last_block_hash() {
        let mut blockchain = Blockchain::new(4);
        let last_hash = blockchain.last_hash();

        assert_eq!(last_hash, blockchain.chain[0].headers.block_hash);
    }

    #[test]
    fn should_create_a_new_block() {
        let mut blockchain = Blockchain::new(4);
        let block = blockchain.create_block(String::from("New Block"));

        assert_eq!(block.previous_hash, blockchain.chain[0].headers.block_hash);
        assert_eq!(block.data, "New Block");
    }
}
