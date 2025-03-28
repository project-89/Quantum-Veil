use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use solana_sdk::pubkey::Pubkey;

/// Glitch Gang NFT metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlitchGangMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub attributes: Vec<Attribute>,
    pub image: String,
    pub properties: Properties,
    /// Private data added by our privacy system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_data: Option<PrivateData>,
}

/// NFT attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

/// NFT properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Properties {
    pub files: Vec<File>,
}

/// NFT file reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub uri: String,
}

/// Private data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateData {
    pub privacy_level: String,
    pub encrypted_attributes: Option<String>,
    pub timeline_fragments: Option<Vec<String>>,
    pub vrm_config: Option<VrmConfig>,
}

/// VRM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmConfig {
    pub model_uri: String,
    pub privacy_settings: HashMap<String, String>,
}

/// VRM position data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// VRM rotation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// VRM voice data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceData {
    pub frequency: Vec<f32>,
    pub amplitude: Vec<f32>,
    pub pitch: f32,
    pub timbre: f32,
}

/// VRM gesture data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureData {
    pub name: String,
    pub intensity: f32,
    pub speed: f32,
    pub joint_rotations: HashMap<String, RotationData>,
}

/// Combined VRM data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmData {
    pub position: PositionData,
    pub rotation: RotationData,
    pub voice: Option<VoiceData>,
    pub gestures: Vec<GestureData>,
    pub animations: HashMap<String, f32>,
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Privacy wrapper instruction enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WrapperInstruction {
    /// Create privacy wrapper for existing NFT
    CreateWrapper {
        /// Initial privacy config hash
        privacy_config_hash: String,
    },
    
    /// Update privacy settings
    UpdatePrivacy {
        /// New privacy config hash
        new_privacy_config_hash: String,
    },
    
    /// Grant access to a specific account
    GrantAccess {
        /// Account to grant access to
        account: String,
        /// Access level (0-255, where 255 is full access)
        level: u8,
    },
    
    /// Revoke access
    RevokeAccess {
        /// Account to revoke access from
        account: String,
    },
}

/// Privacy level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyLevel {
    None = 0,
    Light = 1,
    Medium = 2,
    Heavy = 3,
    Complete = 4,
}

/// Timeline types for metadata fragmentation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimelineType {
    /// Primary timeline (core metadata)
    Primary,
    /// Identity timeline (personal info)
    Identity,
    /// Activity timeline (actions and behaviors)
    Activity,
    /// Social timeline (relationships)
    Social,
    /// Financial timeline (transactions)
    Financial,
    /// Custom timeline
    Custom(String),
}

/// Metadata fragment storage location
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
pub struct SyncMaskConfig {
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
    pub sync_mask: SyncMaskConfig,
}
