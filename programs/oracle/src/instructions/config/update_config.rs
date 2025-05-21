use crate::state::oracle_config::OracleConfig;
use anchor_lang::prelude::*;

pub fn update_oracle_config(ctx: Context<UpdateOracleConfig>, new_admin: Pubkey) -> Result<()> {
    ctx.accounts.oracle_config.admin = new_admin;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateOracleConfig<'info> {
    #[account(
        mut,
        seeds = [b"oracle_config"],
        bump,
        has_one = admin
    )]
    pub oracle_config: Account<'info, OracleConfig>,

    pub admin: Signer<'info>,
}
