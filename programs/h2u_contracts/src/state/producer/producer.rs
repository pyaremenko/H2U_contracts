use anchor_lang::prelude::*;

#[account]
pub struct Producer {
    pub id: u64,
    pub name: String,
    pub authority: Pubkey, // The producer's wallet
}
