use solana_program::{
    program_error::ProgramError,
    msg,
};
use thiserror::Error;

/// Custom error types for the Privacy Wrapper program
#[derive(Error, Debug, Copy, Clone)]
pub enum PrivacyWrapperError {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
    
    /// Not the NFT owner
    #[error("Not the NFT owner")]
    NotNFTOwner,
    
    /// Invalid account data
    #[error("Invalid account data")]
    InvalidAccountData,
    
    /// Account not initialized
    #[error("Account not initialized")]
    AccountNotInitialized,
}

impl From<PrivacyWrapperError> for ProgramError {
    fn from(e: PrivacyWrapperError) -> Self {
        msg!("{}", e);
        ProgramError::Custom(e as u32)
    }
}
