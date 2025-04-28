use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitH2Canister<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"h2_canister", producer.key().as_ref()],
        bump,
        space = 8 + 32 + 8 + 8
    )]
    pub h2_canister: Account<'info, H2Canister>,

    pub producer: Account<'info, Producer>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_h2_canister(ctx: Context<InitH2Canister>) -> Result<()> {
    let canister = &mut ctx.accounts.h2_canister;
    canister.total_amount = 0;
    canister.available_amount = 0;
    canister.producer_pubkey = ctx.accounts.producer.key();
    Ok(())
}