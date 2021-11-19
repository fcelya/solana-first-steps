
use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError{
    //Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Account is not rent exempt")]
    NotRentExempt,
    #[error("Not all expected tokens were transferred")]
    ExpectedAmountMismatch,
    #[error("Amount overflown")]
    AmountOverflow,

}

impl From<EscrowError> for ProgramError{
    fn from(e: EscrowError)-> Self{
        ProgramError::Custom(e as u32)
    }
}