// Entry Point file

pub mod error;
pub mod processor;

// Importing crates
use {
    crate::{error::MintError},
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
        program_error::PrintProgramError, pubkey::Pubkey,
    },
};

// If process instructions fail to execute then throw Mint Error
entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it to figure out what went wrong
        error.print::<MintError>();
        return Err(error);
    }
    Ok(())
}
