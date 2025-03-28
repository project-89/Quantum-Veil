use borsh::{BorshDeserialize, BorshSerialize};

/// Instructions for the Privacy Wrapper program
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum WrapperInstruction {
    /// Create privacy wrapper for existing NFT
    /// 
    /// Accounts expected:
    /// 0. `[signer]` The NFT owner (fee payer)
    /// 1. `[]` The NFT mint account
    /// 2. `[writable]` The new wrapper account
    /// 3. `[]` System program
    /// 4. `[]` Rent sysvar
    CreateWrapper {
        /// Initial privacy config hash
        privacy_config_hash: String,
    },
    
    /// Update privacy settings
    /// 
    /// Accounts expected:
    /// 0. `[signer]` The NFT owner
    /// 1. `[writable]` The wrapper account
    UpdatePrivacy {
        /// New privacy config hash
        new_privacy_config_hash: String,
    },
    
    /// Grant access to a specific account
    /// 
    /// Accounts expected:
    /// 0. `[signer]` The NFT owner
    /// 1. `[writable]` The wrapper account
    GrantAccess {
        /// Account to grant access to
        account: String,
        /// Access level (0-255, where 255 is full access)
        level: u8,
    },
    
    /// Revoke access
    /// 
    /// Accounts expected:
    /// 0. `[signer]` The NFT owner
    /// 1. `[writable]` The wrapper account
    RevokeAccess {
        /// Account to revoke access from
        account: String,
    },
}
