use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to update this PDA.")]
    Unauthorized,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided name is too long. Max 64 characters.")]
    NameTooLong,
}
