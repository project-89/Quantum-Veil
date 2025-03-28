use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::{AccountMeta, Instruction},
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::{str::FromStr, fs};
use std::collections::HashMap;
use base64;
use ring::{digest, hmac};
use rand::{Rng, rngs::OsRng};

use crate::models::{
    GlitchGangMetadata, PrivacyLevel, VrmData, PrivateData, VrmConfig, WrapperInstruction,
    TimelineType, MetadataFragment
};

// Import crate components
use quantum_veil::{QuantumVeil, EntropySource, PrivacyConfig, SynchronicityMask as QVSyncMask};
use synchronicity_mask::{SynchronicityMask};
use timeline_shifter::{TimelineShifter};

/// Privacy Client for Glitch Gang NFTs
pub struct GlitchGangPrivacyClient {
    /// Solana RPC client
    rpc_client: RpcClient,
    /// Owner's keypair
    owner_keypair: Keypair,
    /// Program ID
    program_id: Pubkey,
    /// Quantum Veil encryption system
    quantum_veil: QuantumVeil,
    /// Synchronicity Mask for VRM privacy
    sync_mask: SynchronicityMask,
    /// Timeline Shifter for metadata fragmentation
    timeline_shifter: Option<TimelineShifter>,
    /// Encryption key
    encryption_key: [u8; 32],
}

impl GlitchGangPrivacyClient {
    /// Create a new client
    pub fn new(
        solana_rpc: &str,
        owner_keypair: Keypair,
    ) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            solana_rpc.to_string(),
            CommitmentConfig::confirmed(),
        );
        
        let program_id = Pubkey::from_str("GlchWrapperProgram111111111111111111111111111").unwrap();
        
        let quantum_veil = QuantumVeil::new(solana_rpc);
        let sync_mask = SynchronicityMask::new(solana_rpc);
        
        // Generate a secure encryption key
        let mut encryption_key = [0u8; 32];
        OsRng.fill(&mut encryption_key);
        
        Self {
            rpc_client,
            owner_keypair,
            program_id,
            quantum_veil,
            sync_mask,
            timeline_shifter: None,
            encryption_key,
        }
    }
    
    /// Set a specific encryption key
    pub fn with_encryption_key(mut self, key: [u8; 32]) -> Self {
        self.encryption_key = key;
        self
    }
    
    /// Set the timeline shifter
    pub fn with_timeline_shifter(mut self, shifter: TimelineShifter) -> Self {
        self.timeline_shifter = Some(shifter);
        self
    }
    
    /// Fetch NFT metadata
    pub async fn fetch_metadata(&self, metadata_uri: &str) -> Result<GlitchGangMetadata, String> {
        log::info!("Fetching metadata from: {}", metadata_uri);
        
        let response = reqwest::get(metadata_uri)
            .await
            .map_err(|e| format!("Failed to fetch metadata: {}", e))?;
        
        let metadata: GlitchGangMetadata = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse metadata: {}", e))?;
        
        Ok(metadata)
    }
    
    /// Create privacy wrapper for existing NFT
    pub async fn create_wrapper(
        &self,
        nft_mint: &Pubkey,
        metadata: &GlitchGangMetadata,
    ) -> Result<Pubkey, String> {
        log::info!("Creating privacy wrapper for NFT: {}", nft_mint);
        
        // Create entropy sources
        let entropy_sources = vec![
            EntropySource::BlockchainHash,
            EntropySource::TimeEntropy,
            EntropySource::CosmicNoise,
        ];
        
        // Create synchronicity mask config
        let sync_mask_config = self.sync_mask.create_config(
            nft_mint,
            &self.owner_keypair.pubkey(),
            PrivacyLevel::Medium,
        );
        
        // Create quantum veil config
        let qv_sync_mask = QVSyncMask {
            position_noise: 0.5,
            voice_noise: 0.7,
            gesture_noise: 0.3,
            trusted_agents: Vec::new(),
        };
        
        let privacy_config = self.quantum_veil.create_config(
            &self.owner_keypair.pubkey(),
            nft_mint,
            entropy_sources,
            3600, // Rotate key every hour
            qv_sync_mask,
        );
        
        // Get config hash
        let privacy_config_hash = self.quantum_veil.get_config_hash(&privacy_config);
        
        // Create wrapper account
        let wrapper_account = Keypair::new();
        
        // Prepare instruction
        let instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(self.owner_keypair.pubkey(), true),
                AccountMeta::new_readonly(*nft_mint, false),
                AccountMeta::new(wrapper_account.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
            ],
            data: WrapperInstruction::CreateWrapper {
                privacy_config_hash: privacy_config_hash.clone(),
            }
            .try_to_vec()
            .map_err(|e| format!("Failed to serialize instruction: {}", e))?,
        };
        
        // Create and send transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.owner_keypair.pubkey()),
            &[&self.owner_keypair, &wrapper_account],
            self.rpc_client.get_latest_blockhash().map_err(|e| e.to_string())?,
        );
        
        let signature = self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| format!("Failed to send transaction: {}", e))?;
        
        log::info!("Wrapper created! Signature: {}", signature);
        
        Ok(wrapper_account.pubkey())
    }
    
    /// Apply privacy protections to metadata
    pub async fn protect_metadata(
        &mut self,
        metadata: &GlitchGangMetadata,
        privacy_level: PrivacyLevel,
    ) -> Result<GlitchGangMetadata, String> {
        log::info!("Applying privacy protections to metadata...");
        
        let mut protected_metadata = metadata.clone();
        
        // Select sensitive attributes to protect based on privacy level
        let sensitive_attributes = match privacy_level {
            PrivacyLevel::None => Vec::new(),
            PrivacyLevel::Light => vec!["Secret Code", "Agent Name"],
            PrivacyLevel::Medium => vec!["Secret Code", "Agent Name", "Mission", "Origin"],
            PrivacyLevel::Heavy | PrivacyLevel::Complete => {
                vec!["Secret Code", "Agent Name", "Mission", "Origin", "Accessory", "Symbols"]
            }
        };
        
        // Extract sensitive attributes
        let mut private_attrs = Vec::new();
        let mut public_attrs = Vec::new();
        
        for attr in &metadata.attributes {
            if sensitive_attributes.contains(&attr.trait_type.as_str()) {
                private_attrs.push(attr.clone());
            } else {
                public_attrs.push(attr.clone());
            }
        }
        
        // Replace protected attributes with placeholders
        protected_metadata.attributes = public_attrs;
        
        // Only encrypt if we have sensitive attributes
        if !private_attrs.is_empty() {
            // Encrypt private attributes
            let private_json = serde_json::to_string(&private_attrs)
                .map_err(|e| format!("Failed to serialize private attributes: {}", e))?;
            
            let encrypted = self.encrypt_data(private_json.as_bytes())?;
            let encrypted_b64 = base64::encode(&encrypted);
            
            // Fracture metadata if timeline shifter is available
            let mut timeline_fragments = None;
            if let Some(shifter) = &mut self.timeline_shifter {
                let nft_id = metadata.name.clone();
                
                // Configure timeline distribution
                let mut timeline_config = HashMap::new();
                timeline_config.insert(TimelineType::Primary, 0.3);
                timeline_config.insert(TimelineType::Identity, 0.15);
                timeline_config.insert(TimelineType::Activity, 0.15);
                timeline_config.insert(TimelineType::Social, 0.2);
                timeline_config.insert(TimelineType::Financial, 0.2);
                
                let fragments = shifter.fracture_metadata(
                    &nft_id,
                    private_json.as_bytes(),
                    &self.encryption_key,
                    timeline_config,
                ).await?;
                
                timeline_fragments = Some(fragments);
            }
            
            // Add private data section
            protected_metadata.private_data = Some(PrivateData {
                privacy_level: format!("{:?}", privacy_level),
                encrypted_attributes: Some(encrypted_b64),
                timeline_fragments,
                vrm_config: None,
            });
        }
        
        Ok(protected_metadata)
    }
    
    /// Decrypt protected metadata
    pub fn decrypt_metadata(&self, protected_metadata: &GlitchGangMetadata) -> Result<GlitchGangMetadata, String> {
        log::info!("Decrypting protected metadata...");
        
        let mut decrypted_metadata = protected_metadata.clone();
        
        if let Some(private_data) = &protected_metadata.private_data {
            if let Some(encrypted_b64) = &private_data.encrypted_attributes {
                // Decode base64
                let encrypted = base64::decode(encrypted_b64)
                    .map_err(|e| format!("Failed to decode base64: {}", e))?;
                
                // Decrypt data
                let decrypted = self.decrypt_data(&encrypted)?;
                
                // Parse private attributes
                let private_attrs: Vec<crate::models::Attribute> = serde_json::from_slice(&decrypted)
                    .map_err(|e| format!("Failed to parse private attributes: {}", e))?;
                
                // Add private attributes back to metadata
                decrypted_metadata.attributes.extend(private_attrs);
            }
        }
        
        Ok(decrypted_metadata)
    }
    
    /// Encrypt data
    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Use HMAC as a simple encryption method (in a real system, use ChaCha20Poly1305)
        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.encryption_key);
        let tag = hmac::sign(&key, data);
        
        // Combine tag and data
        let mut encrypted = tag.as_ref().to_vec();
        encrypted.extend_from_slice(data);
        
        Ok(encrypted)
    }
    
    /// Decrypt data
    fn decrypt_data(&self, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        if encrypted.len() < 32 {
            return Err("Encrypted data too short".to_string());
        }
        
        // Separate tag and data
        let tag = &encrypted[0..32];
        let data = &encrypted[32..];
        
        // Verify with HMAC
        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.encryption_key);
        
        match hmac::verify(&key, data, tag) {
            Ok(_) => Ok(data.to_vec()),
            Err(_) => Err("Decryption failed: invalid key or corrupted data".to_string()),
        }
    }
    
    /// Add VRM privacy settings
    pub fn add_vrm_privacy(&mut self, metadata: &mut GlitchGangMetadata, model_uri: &str) -> Result<(), String> {
        log::info!("Adding VRM privacy settings...");
        
        let privacy_settings = HashMap::from([
            ("position".to_string(), "medium".to_string()),
            ("voice".to_string(), "high".to_string()),
            ("gesture".to_string(), "medium".to_string()),
            ("animation".to_string(), "low".to_string()),
        ]);
        
        let vrm_config = VrmConfig {
            model_uri: model_uri.to_string(),
            privacy_settings,
        };
        
        // Create or update private data section
        if metadata.private_data.is_none() {
            metadata.private_data = Some(PrivateData {
                privacy_level: "Medium".to_string(),
                encrypted_attributes: None,
                timeline_fragments: None,
                vrm_config: Some(vrm_config),
            });
        } else {
            metadata.private_data.as_mut().unwrap().vrm_config = Some(vrm_config);
        }
        
        Ok(())
    }
    
    /// Process VRM data with privacy protections
    pub fn process_vrm_data(
        &self,
        vrm_data: &VrmData,
        viewer_id: Option<&str>,
        nft_mint: &Pubkey,
    ) -> Result<VrmData, String> {
        log::info!("Processing VRM data with privacy protections...");
        
        // Apply synchronicity mask
        self.sync_mask.apply_mask(
            &nft_mint.to_string(),
            vrm_data,
            viewer_id,
        )
    }
    
    /// Grant access to a specific account
    pub async fn grant_access(
        &self,
        wrapper_account: &Pubkey,
        account_id: &str,
        access_level: u8,
    ) -> Result<String, String> {
        log::info!("Granting access to {} with level {}...", account_id, access_level);
        
        // Prepare instruction
        let instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(self.owner_keypair.pubkey(), true),
                AccountMeta::new(*wrapper_account, false),
            ],
            data: WrapperInstruction::GrantAccess {
                account: account_id.to_string(),
                level: access_level,
            }
            .try_to_vec()
            .map_err(|e| format!("Failed to serialize instruction: {}", e))?,
        };
        
        // Create and send transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.owner_keypair.pubkey()),
            &[&self.owner_keypair],
            self.rpc_client.get_latest_blockhash().map_err(|e| e.to_string())?,
        );
        
        let signature = self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| format!("Failed to send transaction: {}", e))?;
        
        Ok(signature.to_string())
    }
    
    /// Revoke access
    pub async fn revoke_access(
        &self,
        wrapper_account: &Pubkey,
        account_id: &str,
    ) -> Result<String, String> {
        log::info!("Revoking access from {}...", account_id);
        
        // Prepare instruction
        let instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(self.owner_keypair.pubkey(), true),
                AccountMeta::new(*wrapper_account, false),
            ],
            data: WrapperInstruction::RevokeAccess {
                account: account_id.to_string(),
            }
            .try_to_vec()
            .map_err(|e| format!("Failed to serialize instruction: {}", e))?,
        };
        
        // Create and send transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.owner_keypair.pubkey()),
            &[&self.owner_keypair],
            self.rpc_client.get_latest_blockhash().map_err(|e| e.to_string())?,
        );
        
        let signature = self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| format!("Failed to send transaction: {}", e))?;
        
        Ok(signature.to_string())
    }
    
    /// Update privacy settings
    pub async fn update_privacy_settings(
        &self,
        wrapper_account: &Pubkey,
        new_privacy_config_hash: &str,
    ) -> Result<String, String> {
        log::info!("Updating privacy settings with new hash: {}", new_privacy_config_hash);
        
        // Prepare instruction
        let instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(self.owner_keypair.pubkey(), true),
                AccountMeta::new(*wrapper_account, false),
            ],
            data: WrapperInstruction::UpdatePrivacy {
                new_privacy_config_hash: new_privacy_config_hash.to_string(),
            }
            .try_to_vec()
            .map_err(|e| format!("Failed to serialize instruction: {}", e))?,
        };
        
        // Create and send transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.owner_keypair.pubkey()),
            &[&self.owner_keypair],
            self.rpc_client.get_latest_blockhash().map_err(|e| e.to_string())?,
        );
        
        let signature = self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| format!("Failed to send transaction: {}", e))?;
        
        Ok(signature.to_string())
    }
    
    /// Save protected metadata to file
    pub fn save_metadata_to_file(
        &self, 
        metadata: &GlitchGangMetadata,
        filename: &str
    ) -> Result<(), String> {
        log::info!("Saving metadata to file: {}", filename);
        
        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
        
        fs::write(filename, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }
}
