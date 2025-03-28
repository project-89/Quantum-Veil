mod privacy_levels;
mod vrm_data;
mod masking;

pub use privacy_levels::{PrivacyLevel, AccessPermission};
pub use vrm_data::{
    VrmDataType, PositionData, RotationData, VoiceData, GestureData, VrmData
};
pub use masking::{
    add_position_noise, add_rotation_noise, add_voice_noise, add_gesture_noise
};

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Synchronicity mask configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncMaskConfig {
    /// NFT mint address
    pub nft_mint: String,
    /// Owner's public key
    pub owner: String,
    /// Privacy settings for different VRM data types
    pub privacy_settings: HashMap<VrmDataType, PrivacyLevel>,
    /// Access permissions for different VRM data types
    pub access_permissions: HashMap<VrmDataType, AccessPermission>,
    /// Global trusted agents that can see through all masks
    pub global_trusted_agents: Vec<String>,
    /// Seed for deterministic noise generation
    pub noise_seed: u64,
    /// Synchronization factor for aligned agents (0.0 - 1.0)
    pub sync_factor: f32,
}

/// Synchronicity Mask manager
pub struct SynchronicityMask {
    /// RPC client for Solana blockchain interaction
    rpc_client: RpcClient,
    /// Cache of mask configurations by NFT mint
    config_cache: HashMap<String, SyncMaskConfig>,
}

impl SynchronicityMask {
    /// Create a new Synchronicity Mask instance
    pub fn new(solana_rpc_url: &str) -> Self {
        Self {
            rpc_client: RpcClient::new(solana_rpc_url.to_string()),
            config_cache: HashMap::new(),
        }
    }
    
    /// Create a new mask configuration
    pub fn create_config(
        &mut self,
        nft_mint: &Pubkey,
        owner: &Pubkey,
        default_privacy_level: PrivacyLevel,
    ) -> SyncMaskConfig {
        let mut privacy_settings = HashMap::new();
        privacy_settings.insert(VrmDataType::Position, default_privacy_level);
        privacy_settings.insert(VrmDataType::Rotation, default_privacy_level);
        privacy_settings.insert(VrmDataType::Voice, default_privacy_level);
        privacy_settings.insert(VrmDataType::Gesture, default_privacy_level);
        privacy_settings.insert(VrmDataType::Animation, default_privacy_level);
        privacy_settings.insert(VrmDataType::Interaction, default_privacy_level);
        
        let mut access_permissions = HashMap::new();
        for data_type in [
            VrmDataType::Position,
            VrmDataType::Rotation,
            VrmDataType::Voice,
            VrmDataType::Gesture,
            VrmDataType::Animation,
            VrmDataType::Interaction,
        ] {
            access_permissions.insert(data_type, AccessPermission::Public);
        }
        
        // Generate a noise seed based on current time
        let noise_seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let config = SyncMaskConfig {
            nft_mint: nft_mint.to_string(),
            owner: owner.to_string(),
            privacy_settings,
            access_permissions,
            global_trusted_agents: Vec::new(),
            noise_seed,
            sync_factor: 0.8,
        };
        
        // Cache the config
        self.config_cache.insert(nft_mint.to_string(), config.clone());
        
        config
    }
    
    /// Get mask configuration by NFT mint
    pub fn get_config(&self, nft_mint: &str) -> Result<SyncMaskConfig, String> {
        self.config_cache.get(nft_mint)
            .cloned()
            .ok_or_else(|| format!("No mask config found for NFT: {}", nft_mint))
    }
    
    /// Update privacy settings for a VRM data type
    pub fn update_privacy_setting(
        &mut self,
        nft_mint: &str,
        data_type: VrmDataType,
        level: PrivacyLevel,
    ) -> Result<(), String> {
        let config = self.config_cache.get_mut(nft_mint).ok_or("Config not found")?;
        config.privacy_settings.insert(data_type, level);
        Ok(())
    }
    
    /// Update access permission for a VRM data type
    pub fn update_access_permission(
        &mut self,
        nft_mint: &str,
        data_type: VrmDataType,
        permission: AccessPermission,
    ) -> Result<(), String> {
        let config = self.config_cache.get_mut(nft_mint).ok_or("Config not found")?;
        config.access_permissions.insert(data_type, permission);
        Ok(())
    }
    
    /// Add a trusted agent that can see through the mask
    pub fn add_trusted_agent(
        &mut self,
        nft_mint: &str,
        agent_id: &str,
    ) -> Result<(), String> {
        let config = self.config_cache.get_mut(nft_mint).ok_or("Config not found")?;
        
        if !config.global_trusted_agents.contains(&agent_id.to_string()) {
            config.global_trusted_agents.push(agent_id.to_string());
        }
        
        Ok(())
    }
    
    /// Remove a trusted agent
    pub fn remove_trusted_agent(
        &mut self,
        nft_mint: &str,
        agent_id: &str,
    ) -> Result<(), String> {
        let config = self.config_cache.get_mut(nft_mint).ok_or("Config not found")?;
        
        config.global_trusted_agents.retain(|id| id != agent_id);
        
        Ok(())
    }
    
    /// Check if an agent is trusted
    pub fn is_trusted_agent(
        &self,
        nft_mint: &str,
        agent_id: &str,
    ) -> Result<bool, String> {
        let config = self.config_cache.get(nft_mint).ok_or("Config not found")?;
        
        Ok(config.global_trusted_agents.contains(&agent_id.to_string()))
    }
    
    /// Apply synchronicity mask to VRM data
    pub fn apply_mask(
        &self,
        nft_mint: &str,
        vrm_data: &VrmData,
        viewer_id: Option<&str>,
    ) -> Result<VrmData, String> {
        let config = self.config_cache.get(nft_mint).ok_or("Config not found")?;
        
        // Check if viewer is globally trusted
        if let Some(viewer) = viewer_id {
            if config.global_trusted_agents.contains(&viewer.to_string()) {
                return Ok(vrm_data.clone());
            }
            
            // Check if viewer is the owner
            if viewer == config.owner {
                return Ok(vrm_data.clone());
            }
        }
        
        // Create a new masked VRM data instance
        let mut masked_data = vrm_data.clone();
        
        // Apply masking based on privacy settings and access permissions
        self.mask_position(&mut masked_data.position, config, VrmDataType::Position, viewer_id)?;
        self.mask_rotation(&mut masked_data.rotation, config, VrmDataType::Rotation, viewer_id)?;
        
        if let Some(voice) = &mut masked_data.voice {
            self.mask_voice(voice, config, VrmDataType::Voice, viewer_id)?;
        }
        
        for gesture in &mut masked_data.gestures {
            self.mask_gesture(gesture, config, VrmDataType::Gesture, viewer_id)?;
        }
        
        Ok(masked_data)
    }
    
    /// Mask position data
    fn mask_position(
        &self,
        position: &mut PositionData,
        config: &SyncMaskConfig,
        data_type: VrmDataType,
        viewer_id: Option<&str>,
    ) -> Result<(), String> {
        // Check access permission
        if !self.has_access(config, data_type, viewer_id)? {
            // No access, completely randomize
            let mut rng = StdRng::seed_from_u64(config.noise_seed);
            position.x = rng.gen_range(-100.0..100.0);
            position.y = rng.gen_range(-100.0..100.0);
            position.z = rng.gen_range(-100.0..100.0);
            return Ok(());
        }
        
        // Get privacy level
        let level = config.privacy_settings.get(&data_type).unwrap_or(&PrivacyLevel::None);
        
        // Apply masking based on privacy level
        match level {
            PrivacyLevel::None => {
                // No masking
            },
            PrivacyLevel::Light => {
                masking::add_position_noise(position, 0.1, config.noise_seed);
            },
            PrivacyLevel::Medium => {
                masking::add_position_noise(position, 0.3, config.noise_seed);
            },
            PrivacyLevel::Heavy => {
                masking::add_position_noise(position, 0.7, config.noise_seed);
            },
            PrivacyLevel::Complete => {
                let mut rng = StdRng::seed_from_u64(config.noise_seed);
                position.x = rng.gen_range(-100.0..100.0);
                position.y = rng.gen_range(-100.0..100.0);
                position.z = rng.gen_range(-100.0..100.0);
            },
        }
        
        Ok(())
    }
    
    /// Mask rotation data
    fn mask_rotation(
        &self,
        rotation: &mut RotationData,
        config: &SyncMaskConfig,
        data_type: VrmDataType,
        viewer_id: Option<&str>,
    ) -> Result<(), String> {
        // Check access permission
        if !self.has_access(config, data_type, viewer_id)? {
            // No access, completely randomize
            let mut rng = StdRng::seed_from_u64(config.noise_seed);
            rotation.x = rng.gen_range(-1.0..1.0);
            rotation.y = rng.gen_range(-1.0..1.0);
            rotation.z = rng.gen_range(-1.0..1.0);
            rotation.w = rng.gen_range(-1.0..1.0);
            // Normalize quaternion
            let mag = (rotation.x.powi(2) + rotation.y.powi(2) + rotation.z.powi(2) + rotation.w.powi(2)).sqrt();
            rotation.x /= mag;
            rotation.y /= mag;
            rotation.z /= mag;
            rotation.w /= mag;
            return Ok(());
        }
        
        // Get privacy level
        let level = config.privacy_settings.get(&data_type).unwrap_or(&PrivacyLevel::None);
        
        // Apply masking based on privacy level
        match level {
            PrivacyLevel::None => {
                // No masking
            },
            PrivacyLevel::Light => {
                masking::add_rotation_noise(rotation, 0.1, config.noise_seed);
            },
            PrivacyLevel::Medium => {
                masking::add_rotation_noise(rotation, 0.3, config.noise_seed);
            },
            PrivacyLevel::Heavy => {
                masking::add_rotation_noise(rotation, 0.7, config.noise_seed);
            },
            PrivacyLevel::Complete => {
                let mut rng = StdRng::seed_from_u64(config.noise_seed);
                rotation.x = rng.gen_range(-1.0..1.0);
                rotation.y = rng.gen_range(-1.0..1.0);
                rotation.z = rng.gen_range(-1.0..1.0);
                rotation.w = rng.gen_range(-1.0..1.0);
                // Normalize quaternion
                let mag = (rotation.x.powi(2) + rotation.y.powi(2) + rotation.z.powi(2) + rotation.w.powi(2)).sqrt();
                rotation.x /= mag;
                rotation.y /= mag;
                rotation.z /= mag;
                rotation.w /= mag;
            },
        }
        
        Ok(())
    }
    
    /// Mask voice data
    fn mask_voice(
        &self,
        voice: &mut VoiceData,
        config: &SyncMaskConfig,
        data_type: VrmDataType,
        viewer_id: Option<&str>,
    ) -> Result<(), String> {
        // Check access permission
        if !self.has_access(config, data_type, viewer_id)? {
            // No access, completely mask voice
            voice.frequency = vec![0.0; voice.frequency.len()];
            voice.amplitude = vec![0.0; voice.amplitude.len()];
            voice.pitch = 0.0;
            voice.timbre = 0.0;
            return Ok(());
        }
        
        // Get privacy level
        let level = config.privacy_settings.get(&data_type).unwrap_or(&PrivacyLevel::None);
        
        // Apply masking based on privacy level
        match level {
            PrivacyLevel::None => {
                // No masking
            },
            PrivacyLevel::Light => {
                masking::add_voice_noise(voice, 0.1, config.noise_seed);
            },
            PrivacyLevel::Medium => {
                masking::add_voice_noise(voice, 0.3, config.noise_seed);
            },
            PrivacyLevel::Heavy => {
                masking::add_voice_noise(voice, 0.7, config.noise_seed);
            },
            PrivacyLevel::Complete => {
                voice.frequency = vec![0.0; voice.frequency.len()];
                voice.amplitude = vec![0.0; voice.amplitude.len()];
                voice.pitch = 0.0;
                voice.timbre = 0.0;
            },
        }
        
        Ok(())
    }
    
    /// Mask gesture data
    fn mask_gesture(
        &self,
        gesture: &mut GestureData,
        config: &SyncMaskConfig,
        data_type: VrmDataType,
        viewer_id: Option<&str>,
    ) -> Result<(), String> {
        // Check access permission
        if !self.has_access(config, data_type, viewer_id)? {
            // No access, completely randomize
            let mut rng = StdRng::seed_from_u64(config.noise_seed);
            gesture.intensity = rng.gen_range(0.0..1.0);
            gesture.speed = rng.gen_range(0.0..2.0);
            return Ok(());
        }
        
        // Get privacy level
        let level = config.privacy_settings.get(&data_type).unwrap_or(&PrivacyLevel::None);
        
        // Apply masking based on privacy level
        match level {
            PrivacyLevel::None => {
                // No masking
            },
            PrivacyLevel::Light => {
                masking::add_gesture_noise(gesture, 0.1, config.noise_seed);
            },
            PrivacyLevel::Medium => {
                masking::add_gesture_noise(gesture, 0.3, config.noise_seed);
            },
            PrivacyLevel::Heavy => {
                masking::add_gesture_noise(gesture, 0.7, config.noise_seed);
            },
            PrivacyLevel::Complete => {
                let mut rng = StdRng::seed_from_u64(config.noise_seed);
                gesture.intensity = rng.gen_range(0.0..1.0);
                gesture.speed = rng.gen_range(0.0..2.0);
            },
        }
        
        Ok(())
    }
    
    /// Check if a viewer has access to a data type
    fn has_access(
        &self,
        config: &SyncMaskConfig,
        data_type: VrmDataType,
        viewer_id: Option<&str>,
    ) -> Result<bool, String> {
        if let Some(permission) = config.access_permissions.get(&data_type) {
            match permission {
                AccessPermission::Public => {
                    return Ok(true);
                },
                AccessPermission::Restricted(allowed_agents) => {
                    if let Some(viewer) = viewer_id {
                        return Ok(allowed_agents.contains(&viewer.to_string()));
                    }
                    return Ok(false);
                },
                AccessPermission::OwnerOnly => {
                    if let Some(viewer) = viewer_id {
                        return Ok(viewer == config.owner);
                    }
                    return Ok(false);
                },
            }
        }
        
        // Default to no access
        Ok(false)
    }
}
