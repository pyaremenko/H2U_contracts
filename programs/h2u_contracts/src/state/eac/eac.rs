use anchor_lang::prelude::*;

#[account]
pub struct EAC {
    pub certificate_capacity_kwts: u64,
    pub available_kwts: u64,
    pub burned_kwts: u64,
    pub producer_pubkey: Pubkey,
    pub token_mint: Pubkey,
}

impl EAC {
    pub const MAXLEN: usize = 8 + 8 + 8 + 8 + 8 + 32 + 32;
}
