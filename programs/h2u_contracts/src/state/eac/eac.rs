use anchor_lang::prelude::*;

#[account]
pub struct EAC {
    pub available_amount: u64,
    pub burned_amount: u64,
    pub producer_pubkey: Pubkey,
}
