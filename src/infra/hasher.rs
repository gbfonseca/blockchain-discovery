use sha2::{Digest, Sha256};

pub fn generate_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let result = hasher.finalize();
    let hash_value = format!("{:x}", result);
    hash_value
}
