use solana_client::rpc_client::RpcClient;
use sha3::{Digest, Sha3_512};
use rand::{Rng, rngs::OsRng};
use std::time::{SystemTime, UNIX_EPOCH};

use super::config::EntropySource;

/// Generate a quantum-grade encryption key
pub fn generate_key(sources: &[EntropySource], rpc_client: &RpcClient) -> (Vec<u8>, Vec<u8>) {
    let mut hasher = Sha3_512::new();
    let mut entropy = Vec::new();
    
    // Gather entropy from selected sources
    for source in sources {
        match source {
            EntropySource::BlockchainHash => {
                // Get recent Solana blockhash
                if let Ok(blockhash) = rpc_client.get_latest_blockhash() {
                    entropy.extend_from_slice(&blockhash.as_ref());
                }
            },
            EntropySource::TimeEntropy => {
                // Current system time with nanosecond precision
                if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
                    entropy.extend_from_slice(&time.as_nanos().to_le_bytes());
                }
            },
            EntropySource::CosmicNoise => {
                // Simulated cosmic background radiation
                // In a real implementation, this could use a hardware random number generator
                let mut rng = OsRng;
                let cosmic_bytes: [u8; 64] = rng.gen();
                entropy.extend_from_slice(&cosmic_bytes);
            },
            EntropySource::AgentBehavior => {
                // Use recent agent behavior as entropy
                // In a real implementation, this would use actual behavior metrics
                let mut rng = OsRng;
                let behavior_bytes: [u8; 32] = rng.gen();
                entropy.extend_from_slice(&behavior_bytes);
            },
        }
    }
    
    // Add additional randomness
    let mut rng = OsRng;
    let random_bytes: [u8; 32] = rng.gen();
    entropy.extend_from_slice(&random_bytes);
    
    // Hash the entropy to create the key
    hasher.update(&entropy);
    let result = hasher.finalize();
    
    // Split the hash result into key and nonce
    let key = result[0..32].to_vec();
    let nonce = result[32..44].to_vec();
    
    (key, nonce)
}

/// Generate a secure random seed for key derivation
pub fn generate_random_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    OsRng.fill(&mut seed);
    seed
}

/// Generate a time-based seed for key derivation
pub fn generate_time_based_seed() -> [u8; 32] {
    let mut hasher = Sha3_512::new();
    
    // Use current time with nanosecond precision
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    hasher.update(&time.to_le_bytes());
    
    // Add some random data to prevent predictability
    let mut rng = OsRng;
    let random_bytes: [u8; 16] = rng.gen();
    hasher.update(&random_bytes);
    
    let hash = hasher.finalize();
    
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash[0..32]);
    seed
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_seeds_are_unique() {
        let seed1 = generate_random_seed();
        let seed2 = generate_random_seed();
        
        // Two random seeds should be different
        assert_ne!(seed1, seed2);
        
        // Small chance this could fail if executed within the same nanosecond
        let time_seed1 = generate_time_based_seed();
        std::thread::sleep(std::time::Duration::from_nanos(1));
        let time_seed2 = generate_time_based_seed();
        
        assert_ne!(time_seed1, time_seed2);
    }
}
