use anchor_lang::prelude::*;

#[account]
pub struct OracleConfig {
    pub admin: Pubkey,
}

impl OracleConfig {
    pub const LEN: usize = 8 + 32;
}
