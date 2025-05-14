use anchor_lang::prelude::*;

#[account]
pub struct MarketConfig {
    pub authority: Pubkey, // Who can update or control config
    pub transfer_manager: Pubkey,
    pub transfer_manager_bump: u8, // For [b"signer"]
}
