mod fragment;
mod storage;
mod timeline;

pub use fragment::MetadataFragment;
pub use storage::StorageLocation;
pub use timeline::TimelineType;

use storage::StorageAdapter;

use ring::{digest, hmac};
use futures::future::join_all;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Timeline Shifter for fracturing and retrieving NFT metadata
pub struct TimelineShifter {
    /// Primary storage adapter
    primary_adapter: Box<dyn StorageAdapter + Send + Sync>,
    /// Map of adapters by timeline type
    adapters: HashMap<TimelineType, Box<dyn StorageAdapter + Send + Sync>>,
    /// Cache of fragments by ID
    fragment_cache: HashMap<String, MetadataFragment>,
}

impl TimelineShifter {
    /// Create a new Timeline Shifter with provided adapters
    pub fn new(
        primary_adapter: Box<dyn StorageAdapter + Send + Sync>,
        adapters: HashMap<TimelineType, Box<dyn StorageAdapter + Send + Sync>>,
    ) -> Self {
        Self {
            primary_adapter,
            adapters,
            fragment_cache: HashMap::new(),
        }
    }
    
    /// Generate a unique fragment ID
    fn generate_fragment_id(&self, timeline: &TimelineType, nft_mint: &str, seed: u64) -> String {
        let mut hasher = digest::Context::new(&digest::SHA256);
        
        // Add NFT mint address
        hasher.update(nft_mint.as_bytes());
        
        // Add timeline type
        let timeline_str = format!("{:?}", timeline);
        hasher.update(timeline_str.as_bytes());
        
        // Add timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string();
        hasher.update(timestamp.as_bytes());
        
        // Add seed for deterministic generation
        let seed_bytes = seed.to_le_bytes();
        hasher.update(&seed_bytes);
        
        // Finalize and encode
        let digest = hasher.finish();
        let id = base64::encode(digest.as_ref());
        
        // Truncate to reasonable length
        id[0..16].to_string()
    }
    
    /// Split metadata into fragments across timelines
    pub async fn fracture_metadata(
        &mut self,
        nft_mint: &str,
        metadata: &[u8],
        encryption_key: &[u8],
        timeline_config: HashMap<TimelineType, f32>, // Timeline type -> fragment percentage
    ) -> Result<Vec<String>, String> {
        log::info!("Fracturing metadata across timelines...");
        
        // Validate timeline config
        let total_percentage: f32 = timeline_config.values().sum();
        if (total_percentage - 1.0).abs() > 0.001 {
            return Err("Timeline configuration percentages must sum to 1.0".to_string());
        }
        
        // Create a deterministic RNG for fragment generation
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut rng = StdRng::seed_from_u64(seed);
        
        // Encrypt the full metadata first
        let encrypted_metadata = self.encrypt_data(metadata, encryption_key)?;
        
        // Calculate fragment sizes based on percentages
        let mut fragment_sizes: HashMap<TimelineType, usize> = HashMap::new();
        let total_bytes = encrypted_metadata.len();
        
        for (timeline, percentage) in &timeline_config {
            let size = (total_bytes as f32 * percentage) as usize;
            fragment_sizes.insert(timeline.clone(), size);
        }
        
        // Adjust sizes to ensure we use all bytes
        let allocated_bytes: usize = fragment_sizes.values().sum();
        if allocated_bytes < total_bytes {
            // Add remaining bytes to primary timeline
            if let Some(primary_size) = fragment_sizes.get_mut(&TimelineType::Primary) {
                *primary_size += total_bytes - allocated_bytes;
            } else {
                // If no primary timeline, add to the first one
                if let Some((first_timeline, first_size)) = fragment_sizes.iter_mut().next() {
                    *first_size += total_bytes - allocated_bytes;
                }
            }
        }
        
        // Create fragments
        let mut fragments: Vec<MetadataFragment> = Vec::new();
        let mut offset = 0;
        
        for (timeline, size) in fragment_sizes {
            if size == 0 {
                continue;
            }
            
            // Generate unique ID for this fragment
            let fragment_id = self.generate_fragment_id(&timeline, nft_mint, rng.gen());
            
            // Extract data slice for this fragment
            let end = std::cmp::min(offset + size, encrypted_metadata.len());
            let data = encrypted_metadata[offset..end].to_vec();
            offset = end;
            
            // Choose storage location based on timeline
            let storage_location = match timeline {
                TimelineType::Primary => StorageLocation::Onchain {
                    program_id: "Glch89PrivacyNFTprogramID111111111111111111111".to_string(),
                    account: format!("fragment_{}", fragment_id),
                },
                TimelineType::Financial => StorageLocation::ShadowRealm {
                    access_path: format!("shadow/financial/{}", fragment_id),
                },
                _ => StorageLocation::Ipfs {
                    cid: format!("placeholder_cid_{}", fragment_id), // Will be updated after storage
                },
            };
            
            let fragment = MetadataFragment {
                id: fragment_id,
                timeline: timeline.clone(),
                data,
                links: Vec::new(), // Will be updated after all fragments are created
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                storage_location,
            };
            
            fragments.push(fragment);
        }
        
        // Create links between fragments
        for i in 0..fragments.len() {
            for j in 0..fragments.len() {
                if i != j {
                    fragments[i].links.push(fragments[j].id.clone());
                }
            }
        }
        
        // Store fragments using appropriate adapters
        let mut fragment_ids = Vec::new();
        let mut store_tasks = Vec::new();
        
        for fragment in fragments.iter() {
            fragment_ids.push(fragment.id.clone());
            
            // Get the appropriate adapter for this timeline
            let adapter = if let Some(adapter) = self.adapters.get(&fragment.timeline) {
                adapter
            } else {
                &self.primary_adapter
            };
            
            // Store the fragment
            let fragment_clone = fragment.clone();
            let store_task = async move {
                adapter.store_fragment(&fragment_clone).await
            };
            
            store_tasks.push(store_task);
        }
        
        // Wait for all storage operations to complete
        let results = join_all(store_tasks).await;
        
        // Check for errors
        for result in results {
            if let Err(e) = result {
                return Err(format!("Failed to store fragment: {}", e));
            }
        }
        
        // Cache fragments
        for fragment in fragments {
            self.fragment_cache.insert(fragment.id.clone(), fragment);
        }
        
        Ok(fragment_ids)
    }
    
    /// Reassemble metadata from fragments
    pub async fn reassemble_metadata(
        &mut self,
        fragment_ids: &[String],
        encryption_key: &[u8],
    ) -> Result<Vec<u8>, String> {
        log::info!("Reassembling metadata from {} fragments...", fragment_ids.len());
        
        // Collect fragments
        let mut fragments = Vec::new();
        let mut retrieve_tasks = Vec::new();
        
        for id in fragment_ids {
            // Check cache first
            if let Some(fragment) = self.fragment_cache.get(id) {
                fragments.push(fragment.clone());
                continue;
            }
            
            // Need to retrieve from storage
            let id_clone = id.clone();
            let shifter = self.clone();
            
            let retrieve_task = async move {
                shifter.retrieve_fragment(&id_clone).await
            };
            
            retrieve_tasks.push(retrieve_task);
        }
        
        // Wait for all retrieve operations to complete
        let results = join_all(retrieve_tasks).await;
        
        // Process results
        for result in results {
            match result {
                Ok(fragment) => {
                    fragments.push(fragment.clone());
                    self.fragment_cache.insert(fragment.id.clone(), fragment);
                },
                Err(e) => {
                    return Err(format!("Failed to retrieve fragment: {}", e));
                }
            }
        }
        
        // Sort fragments by timestamp to ensure correct order
        fragments.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        // Combine fragment data
        let mut combined_data = Vec::new();
        for fragment in fragments {
            combined_data.extend_from_slice(&fragment.data);
        }
        
        // Decrypt the combined data
        let decrypted_data = self.decrypt_data(&combined_data, encryption_key)?;
        
        Ok(decrypted_data)
    }
    
    /// Retrieve a specific fragment by ID
    async fn retrieve_fragment(&self, id: &str) -> Result<MetadataFragment, String> {
        log::info!("Retrieving fragment: {}", id);
        
        // Try each adapter until we find the fragment
        for (_, adapter) in &self.adapters {
            if let Ok(true) = adapter.fragment_exists(id).await {
                return adapter.retrieve_fragment(id).await;
            }
        }
        
        // Try primary adapter as fallback
        self.primary_adapter.retrieve_fragment(id).await
    }
    
    /// Encrypt data using the provided key
    fn encrypt_data(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
        // This is a simplified implementation
        // In a real system, use proper encryption like ChaCha20Poly1305
        
        // Use HMAC as a simple encryption method for demonstration
        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        let tag = hmac::sign(&key, data);
        
        // Combine tag and data
        let mut encrypted = tag.as_ref().to_vec();
        encrypted.extend_from_slice(data);
        
        Ok(encrypted)
    }
    
    /// Decrypt data using the provided key
    fn decrypt_data(&self, encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
        // This is a simplified implementation
        // In a real system, use proper decryption like ChaCha20Poly1305
        
        if encrypted.len() < 32 {
            return Err("Encrypted data too short".to_string());
        }
        
        // Separate tag and data
        let tag = &encrypted[0..32];
        let data = &encrypted[32..];
        
        // Verify with HMAC
        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        
        match hmac::verify(&key, data, tag) {
            Ok(_) => Ok(data.to_vec()),
            Err(_) => Err("Decryption failed: invalid key or corrupted data".to_string()),
        }
    }
}

impl Clone for TimelineShifter {
    fn clone(&self) -> Self {
        // Note: This is a simplified clone implementation
        // In a real system, you would need to properly clone the adapters
        Self {
            primary_adapter: self.primary_adapter.clone_adapter(),
            adapters: self.adapters.iter().map(|(k, v)| (k.clone(), v.clone_adapter())).collect(),
            fragment_cache: self.fragment_cache.clone(),
        }
    }
}
