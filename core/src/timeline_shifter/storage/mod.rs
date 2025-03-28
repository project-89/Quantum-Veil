use serde::{Serialize, Deserialize};
use async_trait::async_trait;

// Import sub-modules
pub mod arweave;
pub mod ipfs;
pub mod solana;

// Re-export adapters
pub use arweave::ArweaveAdapter;
pub use ipfs::IpfsAdapter;
pub use solana::SolanaAdapter;

use super::fragment::MetadataFragment;

/// Storage location for metadata fragments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageLocation {
    /// On-chain Solana storage
    Onchain {
        /// Program ID
        program_id: String,
        /// Account address
        account: String,
    },
    /// Arweave permanent storage
    Arweave {
        /// Transaction ID
        transaction_id: String,
    },
    /// IPFS decentralized storage
    Ipfs {
        /// IPFS CID
        cid: String,
    },
    /// Shadow realm (custom encrypted storage)
    ShadowRealm {
        /// Encrypted access info
        access_path: String,
    },
}

/// Storage adapter trait for different timeline fragment storage solutions
#[async_trait]
pub trait StorageAdapter: Send + Sync {
    /// Store a metadata fragment
    async fn store_fragment(&self, fragment: &MetadataFragment) -> Result<String, String>;
    
    /// Retrieve a metadata fragment
    async fn retrieve_fragment(&self, id: &str) -> Result<MetadataFragment, String>;
    
    /// Check if a fragment exists
    async fn fragment_exists(&self, id: &str) -> Result<bool, String>;
    
    /// Delete a fragment
    async fn delete_fragment(&self, id: &str) -> Result<(), String>;
    
    /// Clone the adapter (used for TimelineShifter cloning)
    fn clone_adapter(&self) -> Box<dyn StorageAdapter + Send + Sync>;
}
