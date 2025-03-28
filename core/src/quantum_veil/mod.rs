mod config;
mod encryption;
mod key_gen;

pub use config::{PrivacyConfig, SynchronicityMask, EntropySource};
pub use encryption::{encrypt_data, decrypt_data};
pub use key_gen::generate_key;

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use ring::digest::Context;
use base64::{encode, decode};
use sha3::{Sha3_512, Digest};

/// Quantum Veil encryption manager
pub struct QuantumVeil {
    /// RPC client for Solana blockchain interaction
    rpc_client: RpcClient,
    /// Cache of privacy configurations by NFT mint
    config_cache: HashMap<String, PrivacyConfig>,
}

impl QuantumVeil {
    /// Create a new Quantum Veil instance
    pub fn new(solana_rpc_url: &str) -> Self {
        Self {
            rpc_client: RpcClient::new(solana_rpc_url.to_string()),
            config_cache: HashMap::new(),
        }
    }
    
    /// Create a new privacy configuration for an NFT
    pub fn create_config(
        &mut self,
        owner: &Pubkey,
        nft_mint: &Pubkey,
        entropy_sources: Vec<EntropySource>,
        key_rotation_frequency: u64,
        sync_mask: SynchronicityMask,
    ) -> PrivacyConfig {
        // Generate initial encryption key
        let (key, nonce) = generate_key(&entropy_sources, &self.rpc_client);
        
        let config = PrivacyConfig {
            owner: owner.to_string(),
            nft_mint: nft_mint.to_string(),
            current_key: encode(&key),
            current_nonce: encode(&nonce),
            entropy_sources,
            key_rotation_frequency,
            last_rotation: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            sync_mask,
        };
        
        // Cache the config
        self.config_cache.insert(nft_mint.to_string(), config.clone());
        
        config
    }
    
    /// Get privacy configuration by NFT mint
    pub fn get_config(&self, nft_mint: &str) -> Result<PrivacyConfig, String> {
        self.config_cache.get(nft_mint)
            .cloned()
            .ok_or_else(|| format!("No privacy config found for NFT: {}", nft_mint))
    }
    
    /// Update privacy configuration
    pub fn update_config(&mut self, nft_mint: &str, config: PrivacyConfig) -> Result<(), String> {
        self.config_cache.insert(nft_mint.to_string(), config);
        Ok(())
    }
    
    /// Get privacy configuration hash for Solana storage
    pub fn get_config_hash(&self, config: &PrivacyConfig) -> String {
        let mut hasher = Sha3_512::new();
        let config_json = serde_json::to_string(config).unwrap_or_default();
        hasher.update(config_json.as_bytes());
        let result = hasher.finalize();
        
        encode(&result)
    }
    
    /// Rotate encryption key based on new entropy
    pub fn rotate_key(&mut self, nft_mint: &str) -> Result<PrivacyConfig, String> {
        let config = self.config_cache.get(nft_mint).ok_or("Config not found")?;
        
        // Generate new key from current entropy sources
        let (key, nonce) = generate_key(&config.entropy_sources, &self.rpc_client);
        
        // Update config with new key
        let mut updated_config = config.clone();
        updated_config.current_key = encode(&key);
        updated_config.current_nonce = encode(&nonce);
        updated_config.last_rotation = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Update cache
        self.config_cache.insert(nft_mint.to_string(), updated_config.clone());
        
        Ok(updated_config)
    }
    
    /// Encrypt data using the current privacy key
    pub fn encrypt(&self, nft_mint: &str, data: &[u8]) -> Result<Vec<u8>, String> {
        let config = self.config_cache.get(nft_mint).ok_or("Config not found")?;
        
        let key_bytes = decode(&config.current_key).map_err(|_| "Invalid key")?;
        let nonce_bytes = decode(&config.current_nonce).map_err(|_| "Invalid nonce")?;
        
        encryption::encrypt_data(data, &key_bytes, &nonce_bytes)
    }
    
    /// Decrypt data using the current privacy key
    pub fn decrypt(&self, nft_mint: &str, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let config = self.config_cache.get(nft_mint).ok_or("Config not found")?;
        
        let key_bytes = decode(&config.current_key).map_err(|_| "Invalid key")?;
        let nonce_bytes = decode(&config.current_nonce).map_err(|_| "Invalid nonce")?;
        
        encryption::decrypt_data(ciphertext, &key_bytes, &nonce_bytes)
    }
    
    /// Update synchronicity mask
    pub fn update_sync_mask(
        &mut self,
        nft_mint: &str,
        new_mask: SynchronicityMask,
    ) -> Result<PrivacyConfig, String> {
        let mut config = self.get_config(nft_mint)?;
        
        config.sync_mask = new_mask;
        config.last_rotation = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.config_cache.insert(nft_mint.to_string(), config.clone());
        
        Ok(config)
    }
}
