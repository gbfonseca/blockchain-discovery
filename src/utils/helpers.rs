pub fn validate_hash(block_hash_pow: &str, pow_prefix: &str, difficulty: usize) -> bool {
    let check = pow_prefix.repeat(difficulty);
    block_hash_pow.starts_with(&check)
}
