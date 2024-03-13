use chrono::Utc;

use crate::{infra::hasher::generate_hash, utils::helpers::validate_hash};

use super::block::{Block, Headers, Payload};

#[derive(Debug)]
pub struct Blockchain {
    pub difficulty: usize,
    pub chain: Vec<Block>,
    pub pow_prefix: String,
}

#[allow(dead_code)]
impl Blockchain {
    pub fn new(difficulty: usize) -> Blockchain {
        let mut blockchain = Blockchain {
            difficulty,
            chain: Vec::new(),
            pow_prefix: String::from("0"),
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

    pub fn mine_block(&mut self, block: &Payload) -> Option<Block> {
        let mut nonce = 0;
        let start = Utc::now().timestamp();
        loop {
            let block_serialize = serde_json::to_string(&block).unwrap();
            let block_hash = generate_hash(&block_serialize);
            let block_hash_pow = generate_hash(&format!("{}{}", block_hash, nonce));

            if validate_hash(&block_hash_pow, &self.pow_prefix, self.difficulty) {
                let end = Utc::now().timestamp();
                let time_result = end - start;
                println!(
                    "Bloco #{} minerado em {}s. Hash {} ({} tentativas)",
                    block.seq, time_result, block_hash, nonce
                );

                let new_block = Block {
                    headers: Headers { block_hash, nonce },
                    payload: block.to_owned(),
                };
                return Some(new_block);
            }
            nonce += 1;
        }
    }

    pub fn send_block(&mut self, block: &Block) -> Vec<Block> {
        if self.verify_block(block) {
            let block_value = block.clone();
            self.chain.push(block_value);
            println!(
                "Bloco #{} foi adicionado a blockhain {:?}",
                block.payload.seq, self
            )
        }
        self.chain.to_owned()
    }

    fn verify_block(&mut self, block: &Block) -> bool {
        let last_block_hash = self.last_hash();
        if block.payload.previous_hash != last_block_hash {
            eprintln!(
                "Bloco #{} invalido. O Hash anterior e {}",
                block.payload.seq, last_block_hash
            );
            return false;
        }
        let block_serialize = serde_json::to_string(&block.payload).unwrap();
        let test_hash = generate_hash(&format!(
            "{}{}",
            generate_hash(&block_serialize),
            block.headers.nonce
        ));
        if !validate_hash(&test_hash, &self.pow_prefix, self.difficulty) {
            eprintln!(
                "Bloco #{} invalido. Nonce {} invalido e nao pode ser verificado.",
                block.payload.seq, block.headers.nonce
            );
            return false;
        }

        true
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

    #[test]
    fn should_mine_the_new_block() {
        let mut blockchain = Blockchain::new(4);
        let block = blockchain.create_block(String::from("New Block"));
        let mine_info = blockchain.mine_block(&block).unwrap();
        assert_eq!(mine_info.payload.data, block.data);
    }

    #[test]
    fn should_send_block() {
        let mut blockchain = Blockchain::new(4);
        let block = blockchain.create_block(String::from("New Block on Chain"));
        let mine_info = blockchain.mine_block(&block).unwrap();
        let chain = blockchain.send_block(&mine_info);
        assert_eq!(mine_info.payload.data, chain.last().unwrap().payload.data);
    }
}
