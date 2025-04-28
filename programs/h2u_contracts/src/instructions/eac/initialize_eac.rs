use anchor_lang::prelude::*;

use crate::state::{eac::eac::EAC, producer::Producer};

pub fn init_eac_storage(ctx: Context<InitEacStorage>) -> Result<()> {
    let eac = &mut ctx.accounts.eac;
    eac.total_amount = 0;
    eac.available_amount = 0;
    eac.producer_pubkey = ctx.accounts.producer.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitEacStorage<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"eac", producer.key().as_ref()],
        bump,
        space = 8 + 32 + 8 + 8 + 8
    )]
    pub eac: Account<'info, EAC>,

    pub producer: Account<'info, Producer>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
