pub mod client;
pub mod models;

pub use client::GlitchGangPrivacyClient;
pub use models::{
    GlitchGangMetadata,
    PrivacyLevel,
    VrmData,
    PositionData,
    RotationData,
    VoiceData,
    GestureData,
};

/// Project 89: Quantum Veil Privacy System
///
/// A privacy-enhancing system for Glitch Gang NFTs on Solana blockchain.
/// 
/// Features:
/// - Privacy wrappers for existing NFTs
/// - Quantum-grade encryption for sensitive data
/// - Synchronicity masks for VRM behavior obfuscation
/// - Timeline shifting for metadata fragmentation
/// 
/// Developed by the Glitch Gang collective.
#[allow(unused)]
static BANNER: &str = r#"
  _____           _           _     ___  ___  
 |  __ \         (_)         | |    |  \/  |  
 | |__) | __ ___  _  ___  ___| |_   | .  . |  
 |  ___/ '__/ _ \| |/ _ \/ __| __|  | |\/| |  
 | |   | | | (_) | |  __/ (__| |_   | |  | |  
 |_|   |_|  \___/| |\___|\___|\__|  \_|  |_/  
               _/ |                           
              |__/                            
"#;
