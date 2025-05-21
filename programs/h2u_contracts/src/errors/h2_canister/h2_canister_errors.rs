use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid mint amount")]
    InvalidMintAmount,
    #[msg("Batch ID exceeds maximum length of 36 bytes")]
    BatchIdTooLong,
}
