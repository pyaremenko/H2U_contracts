use anchor_lang::prelude::*;

#[account]
pub struct EAC {
    pub available_kwts: u64,
    pub burned_kwts: u64,
    pub available_hydrogen: u64,
    pub producer_pubkey: Pubkey,
}
