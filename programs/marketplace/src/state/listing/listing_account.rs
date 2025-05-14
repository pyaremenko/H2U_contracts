use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub producer: Pubkey,
    pub h2_canister: Pubkey,
    pub price: u64,
    pub transfer_manager_ata: Pubkey,
}
