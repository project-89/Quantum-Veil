// Privacy Wrapper for existing NFTs
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program::{invoke, invoke_signed},
    sysvar::{rent::Rent, Sysvar},
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

// Program ID
solana_program::declare_id!("GlchWrapperProgram111111111111111111111111111");

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PrivacyWrapper {
    // Original NFT mint address
    pub original_nft_mint: Pubkey,
    // Owner of the NFT
    pub owner: Pubkey,
    // Privacy config hash (points to off-chain privacy settings)
    pub privacy_config_hash: String,
    // Mapping of access levels per account
    pub access_controls: HashMap<String, u8>,
    // Last update timestamp
    pub last_updated: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum WrapperInstruction {
    /// Create privacy wrapper for existing NFT
    /// 
    /// Accounts expected:
    /// 0. `[signer]` The NFT owner (fee payer)
    /// 1. `[]` The NFT mint account
    /// 2. `[writable]` The new wrapper account
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

// Program entrypoint
entrypoint!(process_instruction);

// Program logic
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = WrapperInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        WrapperInstruction::CreateWrapper { privacy_config_hash } => {
            create_wrapper(program_id, accounts, privacy_config_hash)
        }
        WrapperInstruction::UpdatePrivacy { new_privacy_config_hash } => {
            update_privacy(program_id, accounts, new_privacy_config_hash)
        }
        WrapperInstruction::GrantAccess { account, level } => {
            grant_access(program_id, accounts, account, level)
        }
        WrapperInstruction::RevokeAccess { account } => {
            revoke_access(program_id, accounts, account)
        }
    }
}

/// Create a new privacy wrapper
pub fn create_wrapper(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    privacy_config_hash: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let owner = next_account_info(account_info_iter)?;
    let nft_mint = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Create the wrapper data
    let wrapper = PrivacyWrapper {
        original_nft_mint: *nft_mint.key,
        owner: *owner.key,
        privacy_config_hash,
        access_controls: HashMap::new(),
        last_updated: solana_program::clock::Clock::get()?.unix_timestamp as u64,
    };
    
    // Serialize and store the wrapper
    wrapper.serialize(&mut *wrapper_account.data.borrow_mut())?;
    
    msg!("Privacy wrapper created for NFT: {}", nft_mint.key);
    
    Ok(())
}

/// Update privacy settings
pub fn update_privacy(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_privacy_config_hash: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let owner = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Deserialize the wrapper account
    let mut wrapper = PrivacyWrapper::try_from_slice(&wrapper_account.data.borrow())?;
    
    // Verify ownership
    if wrapper.owner != *owner.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Update the privacy config hash
    wrapper.privacy_config_hash = new_privacy_config_hash;
    wrapper.last_updated = solana_program::clock::Clock::get()?.unix_timestamp as u64;
    
    // Save the updated wrapper
    wrapper.serialize(&mut *wrapper_account.data.borrow_mut())?;
    
    msg!("Privacy settings updated for NFT: {}", wrapper.original_nft_mint);
    
    Ok(())
}

/// Grant access to a specific account
pub fn grant_access(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account: String,
    level: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let owner = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Deserialize the wrapper account
    let mut wrapper = PrivacyWrapper::try_from_slice(&wrapper_account.data.borrow())?;
    
    // Verify ownership
    if wrapper.owner != *owner.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Update access control
    wrapper.access_controls.insert(account.clone(), level);
    wrapper.last_updated = solana_program::clock::Clock::get()?.unix_timestamp as u64;
    
    // Save the updated wrapper
    wrapper.serialize(&mut *wrapper_account.data.borrow_mut())?;
    
    msg!("Access granted to {} with level {}", account, level);
    
    Ok(())
}

/// Revoke access from a specific account
pub fn revoke_access(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let owner = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Deserialize the wrapper account
    let mut wrapper = PrivacyWrapper::try_from_slice(&wrapper_account.data.borrow())?;
    
    // Verify ownership
    if wrapper.owner != *owner.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Remove access
    wrapper.access_controls.remove(&account);
    wrapper.last_updated = solana_program::clock::Clock::get()?.unix_timestamp as u64;
    
    // Save the updated wrapper
    wrapper.serialize(&mut *wrapper_account.data.borrow_mut())?;
    
    msg!("Access revoked from {}", account);
    
    Ok(())
}
