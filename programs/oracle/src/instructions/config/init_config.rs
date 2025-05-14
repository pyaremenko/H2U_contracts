use crate::state::{OracleConfig, OraclePrice};
use anchor_lang::prelude::*;

pub fn initialize_oracle_config(ctx: Context<InitializeOracleConfig>) -> Result<()> {
    ctx.accounts.oracle_config.admin = ctx.accounts.authority.key();
    let price = &mut ctx.accounts.oracle_price;
    price.min_price_per_kg = 0;
    price.max_price_per_kg = 0;
    price.last_updated = Clock::get()?.unix_timestamp;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeOracleConfig<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"oracle_price"],
        bump,
        space = OraclePrice::LEN
    )]
    pub oracle_price: Account<'info, OraclePrice>,

    #[account(
        init,
        payer = authority,
        seeds = [b"oracle_config"],
        bump,
        space = OracleConfig::LEN
    )]
    pub oracle_config: Account<'info, OracleConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
