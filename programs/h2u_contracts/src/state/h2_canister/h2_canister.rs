use anchor_lang::prelude::*;

#[account]
pub struct H2Canister {
    pub batch_id: String,
    pub total_amount: u64,
    pub available_hydrogen: u64,
    pub producer_pubkey: Pubkey, // Reference to producer
    pub token_mint: Pubkey,      //reference to created token address
}
