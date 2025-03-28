use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey,
    program_error::ProgramError,
};
use std::collections::HashMap;

/// Privacy wrapper state structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PrivacyWrapper {
    /// Original NFT mint address
    pub original_nft_mint: Pubkey,
    /// Owner of the NFT
    pub owner: Pubkey,
    /// Privacy config hash (points to off-chain privacy settings)
    pub privacy_config_hash: String,
    /// Mapping of access levels per account
    pub access_controls: HashMap<String, u8>,
    /// Last update timestamp
    pub last_updated: u64,
}

impl PrivacyWrapper {
    /// Get the size of the wrapper account
    pub fn get_account_size(privacy_config_hash: &str) -> usize {
        // Calculate size based on struct fields:
        // - Pubkey size (32 bytes) * 2 (original_nft_mint + owner)
        // - String length (4 bytes) + privacy_config_hash bytes
        // - HashMap size (estimated as 4 bytes for len + potential entries)
        // - Timestamp (8 bytes)
        let estimated_access_controls_size = 100; // Allow space for some access entries
        
        (32 * 2) + // Pubkeys
        (4 + privacy_config_hash.len()) + // String length prefix + content
        estimated_access_controls_size +
        8 // Timestamp
    }
    
    /// Check if the account is the owner
    pub fn is_owner(&self, account: &Pubkey) -> bool {
        self.owner == *account
    }
    
    /// Get access level for an account
    pub fn get_access_level(&self, account: &str) -> u8 {
        *self.access_controls.get(account).unwrap_or(&0)
    }
    
    /// Check if an account has required access level
    pub fn has_access(&self, account: &str, required_level: u8) -> bool {
        let account_level = self.get_access_level(account);
        account_level >= required_level
    }
}
