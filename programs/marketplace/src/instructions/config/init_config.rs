use crate::state::config::MarketConfig;
use anchor_lang::prelude::*;

pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.authority.key();
    config.transfer_manager = ctx.accounts.transfer_manager.key();
    config.transfer_manager_bump = ctx.bumps.transfer_manager;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct InitConfig<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"config"],
        bump,
        space = 8 + 32 + 1 + 32
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK:unchecked
    #[account(
        init,
        seeds = [b"transfer_manager"],
        bump,
        payer = authority,
        space = 0,
    )]
    pub transfer_manager: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
