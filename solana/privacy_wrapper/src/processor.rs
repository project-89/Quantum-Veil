use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program::{invoke, invoke_signed},
    sysvar::{rent::Rent, Sysvar},
    clock::Clock,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    instruction::WrapperInstruction,
    state::PrivacyWrapper,
    error::PrivacyWrapperError,
};

/// Program logic entry point
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize instruction
    let instruction = WrapperInstruction::try_from_slice(instruction_data)
        .map_err(|_| PrivacyWrapperError::InvalidInstruction)?;

    // Route to the appropriate instruction handler
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
    
    // Parse accounts
    let owner = next_account_info(account_info_iter)?;
    let nft_mint = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(PrivacyWrapperError::NotNFTOwner.into());
    }
    
    // Calculate space needed
    let space = PrivacyWrapper::get_account_size(&privacy_config_hash);
    
    // Get rent
    let rent = &Rent::from_account_info(rent_info)?;
    let rent_lamports = rent.minimum_balance(space);
    
    // Create account
    invoke(
        &system_instruction::create_account(
            owner.key,
            wrapper_account.key,
            rent_lamports,
            space as u64,
            program_id,
        ),
        &[
            owner.clone(),
            wrapper_account.clone(),
            system_program.clone(),
        ],
    )?;
    
    // Create the wrapper data
    let wrapper = PrivacyWrapper {
        original_nft_mint: *nft_mint.key,
        owner: *owner.key,
        privacy_config_hash,
        access_controls: std::collections::HashMap::new(),
        last_updated: Clock::get()?.unix_timestamp as u64,
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
    
    // Parse accounts
    let owner = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(PrivacyWrapperError::NotNFTOwner.into());
    }
    
    // Verify account ownership
    if wrapper_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize the wrapper account
    let mut wrapper = PrivacyWrapper::try_from_slice(&wrapper_account.data.borrow())
        .map_err(|_| PrivacyWrapperError::InvalidAccountData)?;
    
    // Verify ownership
    if wrapper.owner != *owner.key {
        return Err(PrivacyWrapperError::NotNFTOwner.into());
    }
    
    // Update the privacy config hash
    wrapper.privacy_config_hash = new_privacy_config_hash;
    wrapper.last_updated = Clock::get()?.unix_timestamp as u64;
    
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
    
    // Parse accounts
    let owner = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(PrivacyWrapperError::NotNFTOwner.into());
    }
    
    // Verify account ownership
    if wrapper_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize the wrapper account
    let mut wrapper = PrivacyWrapper::try_from_slice(&wrapper_account.data.borrow())
        .map_err(|_| PrivacyWrapperError::InvalidAccountData)?;
    
    // Verify ownership
    if wrapper.owner != *owner.key {
        return Err(PrivacyWrapperError::NotNFTOwner.into());
    }
    
    // Update access control
    wrapper.access_controls.insert(account.clone(), level);
    wrapper.last_updated = Clock::get()?.unix_timestamp as u64;
    
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
    
    // Parse accounts
    let owner = next_account_info(account_info_iter)?;
    let wrapper_account = next_account_info(account_info_iter)?;
    
    // Verify the owner signed the transaction
    if !owner.is_signer {
        return Err(PrivacyWrapperError::NotNFTOwner.into());
    }
    
    // Verify account ownership
    if wrapper_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize the wrapper account
    let mut wrapper = PrivacyWrapper::try_from_slice(&wrapper_account.data.borrow())
        .map_err(|_| PrivacyWrapperError::InvalidAccountData)?;
    
    // Verify ownership
    if wrapper.owner != *owner.key {
        return Err(PrivacyWrapperError::NotNFTOwner.into());
    }
    
    // Remove access
    wrapper.access_controls.remove(&account);
    wrapper.last_updated = Clock::get()?.unix_timestamp as u64;
    
    // Save the updated wrapper
    wrapper.serialize(&mut *wrapper_account.data.borrow_mut())?;
    
    msg!("Access revoked from {}", account);
    
    Ok(())
}
