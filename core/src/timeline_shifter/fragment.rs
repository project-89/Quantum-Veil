use serde::{Serialize, Deserialize};

use super::timeline::TimelineType;
use super::storage::StorageLocation;

/// Metadata fragment with timeline association
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataFragment {
    /// Fragment ID
    pub id: String,
    /// Timeline this fragment belongs to
    pub timeline: TimelineType,
    /// Encrypted data content
    pub data: Vec<u8>,
    /// Hash linking to other fragments
    pub links: Vec<String>,
    /// Timestamp when fragment was created
    pub timestamp: u64,
    /// Fragment storage location
    pub storage_location: StorageLocation,
}

impl MetadataFragment {
    /// Get the size of the fragment data
    pub fn data_size(&self) -> usize {
        self.data.len()
    }
    
    /// Check if this fragment is linked to another fragment
    pub fn is_linked_to(&self, fragment_id: &str) -> bool {
        self.links.contains(&fragment_id.to_string())
    }
    
    /// Add a link to another fragment
    pub fn add_link(&mut self, fragment_id: &str) {
        if !self.links.contains(&fragment_id.to_string()) {
            self.links.push(fragment_id.to_string());
        }
    }
    
    /// Remove a link to another fragment
    pub fn remove_link(&mut self, fragment_id: &str) {
        self.links.retain(|id| id != fragment_id);
    }
    
    /// Get the fragment storage type
    pub fn storage_type(&self) -> &'static str {
        match &self.storage_location {
            StorageLocation::Onchain { .. } => "Onchain",
            StorageLocation::Arweave { .. } => "Arweave",
            StorageLocation::Ipfs { .. } => "IPFS",
            StorageLocation::ShadowRealm { .. } => "ShadowRealm",
        }
    }
    
    /// Calculate storage cost (theoretical, based on storage type and size)
    pub fn storage_cost(&self) -> f64 {
        let bytes = self.data.len() as f64;
        
        match &self.storage_location {
            StorageLocation::Onchain { .. } => {
                // Solana storage is expensive, cost per byte
                bytes * 0.00001 // SOL per byte (fictional value)
            },
            StorageLocation::Arweave { .. } => {
                // Arweave is one-time permanent storage
                bytes * 0.000001 // AR per byte (fictional value)
            },
            StorageLocation::Ipfs { .. } => {
                // IPFS has ongoing pinning costs
                bytes * 0.0000001 // USD per byte per month (fictional value)
            },
            StorageLocation::ShadowRealm { .. } => {
                // Custom storage, potentially free or using privacy-focused approach
                0.0
            },
        }
    }
}
