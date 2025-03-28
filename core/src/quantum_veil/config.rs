use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Entropy sources for quantum-grade key generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropySource {
    /// Recent Solana blockchain hash
    BlockchainHash,
    /// System time-based entropy
    TimeEntropy,
    /// Cosmic background radiation simulation
    CosmicNoise,
    /// VRM agent interaction data
    AgentBehavior,
}

/// Synchronicity mask settings for VRM behavior obfuscation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronicityMask {
    /// Level of noise added to position data (0.0 - 1.0)
    pub position_noise: f32,
    /// Level of noise added to voice data (0.0 - 1.0)
    pub voice_noise: f32,
    /// Level of noise added to gesture data (0.0 - 1.0)
    pub gesture_noise: f32,
    /// Trusted agents that can see through the mask
    pub trusted_agents: Vec<String>,
}

/// Privacy configuration for a Glitch Gang NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Owner's public key
    pub owner: String,
    /// NFT mint address
    pub nft_mint: String,
    /// Current encryption key (base64 encoded)
    pub current_key: String,
    /// Nonce for encryption (base64 encoded)
    pub current_nonce: String,
    /// Entropy sources used for key generation
    pub entropy_sources: Vec<EntropySource>,
    /// Key rotation frequency in seconds
    pub key_rotation_frequency: u64,
    /// Last key rotation timestamp
    pub last_rotation: u64,
    /// Synchronicity mask settings
    pub sync_mask: SynchronicityMask,
}

impl PrivacyConfig {
    /// Check if key rotation is needed
    pub fn needs_rotation(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        now - self.last_rotation > self.key_rotation_frequency
    }
    
    /// Get the time until next scheduled rotation
    pub fn time_until_next_rotation(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let time_since_last = now - self.last_rotation;
        
        if time_since_last >= self.key_rotation_frequency {
            0
        } else {
            self.key_rotation_frequency - time_since_last
        }
    }
    
    /// Check if the given agent is trusted
    pub fn is_agent_trusted(&self, agent_id: &str) -> bool {
        self.sync_mask.trusted_agents.contains(&agent_id.to_string())
    }
    
    /// Add a trusted agent
    pub fn add_trusted_agent(&mut self, agent_id: &str) {
        if !self.sync_mask.trusted_agents.contains(&agent_id.to_string()) {
            self.sync_mask.trusted_agents.push(agent_id.to_string());
        }
    }
    
    /// Remove a trusted agent
    pub fn remove_trusted_agent(&mut self, agent_id: &str) {
        self.sync_mask.trusted_agents.retain(|id| id != agent_id);
    }
}
