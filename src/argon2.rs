use anyhow::Result;
use argon2::{Algorithm, Argon2, ParamsBuilder, Version};

use crate::messages::Argon2Params;

/// Hash a nonce using Argon2d according to pk910 faucet parameters
///
/// This is the CPU-only baseline implementation (no AVX2 SIMD yet).
/// Each hash allocates ~4MB of memory. We offload this to `spawn_blocking`
/// to avoid blocking the Tokio runtime.
pub fn hash_nonce(pre_image: &[u8], nonce: &[u8], params: &Argon2Params) -> Result<Vec<u8>> {
    // Build Argon2 parameters from server config
    let params_built = ParamsBuilder::new()
        .m_cost(params.memory_cost)
        .t_cost(params.time_cost)
        .p_cost(params.parallelism)
        .output_len(params.key_length as usize)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build Argon2 params: {}", e))?;

    // Determine algorithm variant (0 = Argon2d, 1 = Argon2i, 2 = Argon2id)
    let algorithm = match params.variant {
        0 => Algorithm::Argon2d,
        1 => Algorithm::Argon2i,
        2 => Algorithm::Argon2id,
        _ => anyhow::bail!("Unknown Argon2 variant: {}", params.variant),
    };

    // Determine version (should be 0x13 for v1.3)
    let version = match params.version {
        0x10 => Version::V0x10,
        0x13 => Version::V0x13,
        _ => anyhow::bail!("Unknown Argon2 version: {:#x}", params.version),
    };

    let argon2 = Argon2::new(algorithm, version, params_built);

    // Combine pre_image and nonce as input
    let mut input = Vec::with_capacity(pre_image.len() + nonce.len());
    input.extend_from_slice(pre_image);
    input.extend_from_slice(nonce);

    // Allocate output buffer
    let mut output = vec![0u8; params.key_length as usize];

    // Hash with no salt (empty slice)
    argon2
        .hash_password_into(&input, &[], &mut output)
        .map_err(|e| anyhow::anyhow!("Argon2 hashing failed: {}", e))?;

    Ok(output)
}

/// Check if a hash meets the difficulty target
///
/// The hash must be less than the target (hexadecimal comparison).
pub fn meets_target(hash: &[u8], target: &str) -> Result<bool> {
    let hash_hex = hex::encode(hash);

    // Lexicographic comparison works for hex strings of equal length
    // Target should be a 64-character hex string (32 bytes)
    Ok(hash_hex.as_str() <= target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2d_baseline() {
        let params = Argon2Params {
            variant: 0, // Argon2d
            version: 0x13,
            time_cost: 1,
            memory_cost: 4096, // 4MB
            parallelism: 1,
            key_length: 32,
        };

        let pre_image = b"test_pre_image";
        let nonce = b"0000000000000001";

        let result = hash_nonce(pre_image, nonce, &params);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_meets_target() {
        let hash = hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
        let target = "0000000000000000000000000000000000000000000000000000000000000010";

        assert!(meets_target(&hash, target).unwrap());

        let high_hash =
            hex::decode("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0")
                .unwrap();
        assert!(!meets_target(&high_hash, target).unwrap());
    }
}
