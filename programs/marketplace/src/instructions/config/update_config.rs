use crate::state::config::MarketConfig;
use anchor_lang::prelude::*; // Add your custom error definitions here

pub fn update_config(ctx: Context<UpdateConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.transfer_manager_bump,
        has_one = authority
    )]
    pub config: Account<'info, MarketConfig>,

    pub authority: Signer<'info>,
}
