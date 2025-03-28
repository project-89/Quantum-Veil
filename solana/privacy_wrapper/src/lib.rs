use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// Export modules
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Program ID
solana_program::declare_id!("GlchWrapperProgram111111111111111111111111111");

// Program entrypoint
entrypoint!(process_instruction);

/// Process instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}
