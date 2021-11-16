use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;


pub enum EscrowInstruction{

    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    ///               Needed to sign the transferrance of ownership of the temporary account
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program


    InitEscrow{
        // What A expects to receive of token Y
        amount: u64
    }
}


impl EscrowInstruction{
    // Unpack the byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html)
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError>{
        let (tag,rest) = imput.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag{
            0 => Self::InitEscrow{
                amount: Self::unpack_amount(rest)?, //? makes the fn return the error if the output of the call is not Ok. The type of error returned is the type specified in the fn unpack()
            },
            _=> return Err(InvalidInstruction.into())
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError>{
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}