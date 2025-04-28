use anchor_lang::prelude::*;

#[account]
pub struct EAC {
    pub total_amount: u64,
    pub available_amount: u64,
    pub burned_amount: u64, 
    pub producer: Pubkey, // Reference to producer
}