use anchor_lang::prelude::*;

#[account]
pub struct OraclePrice {
    pub min_price_per_kg: u64,
    pub max_price_per_kg: u64,
    pub last_updated: i64,
}

impl OraclePrice {
    pub const LEN: usize = 8 + 8 + 8 + 8; // 8 discriminator + 3 fields
}
