use crate::state::{OracleConfig, OraclePrice};
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Minimum price must be less than maximum price.")]
    InvalidPriceRange,
}

pub fn update_oracle_price(
    ctx: Context<UpdateOraclePrice>,
    new_min: u64,
    new_max: u64,
) -> Result<()> {
    require!(new_min < new_max, ErrorCode::InvalidPriceRange);

    let price = &mut ctx.accounts.oracle_price;
    price.min_price_per_kg = new_min;
    price.max_price_per_kg = new_max;
    price.last_updated = Clock::get()?.unix_timestamp;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateOraclePrice<'info> {
    #[account(seeds = [b"oracle_config"], bump)]
    pub oracle_config: Account<'info, OracleConfig>,

    #[account(mut, seeds = [b"oracle_price"], bump)]
    pub oracle_price: Account<'info, OraclePrice>,

    #[account(address = oracle_config.admin)]
    pub admin: Signer<'info>,
}
