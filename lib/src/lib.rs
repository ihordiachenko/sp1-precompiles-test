use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bytes message;
        bool use_precompile;
        bytes32 hash;
    }
}

pub fn sha256_with_precompile(input: &[u8]) -> [u8; 32] {
    use patched_sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(input);
    let mut result = [0u8; 32];
    result.copy_from_slice(&hasher.finalize());

    result
}

pub fn sha256(input: &[u8]) -> [u8; 32] {
    use unpatched_sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(input);
    let mut result = [0u8; 32];
    result.copy_from_slice(&hasher.finalize());

    result
}
