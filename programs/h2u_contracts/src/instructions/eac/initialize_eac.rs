use anchor_lang::prelude::*;

use crate::{
    errors::producer::producer_errors::CustomError, state::eac::eac::EAC, state::producer::Producer,
};
//use crate::state::{eac::eac::EAC, producer::Producer};

pub fn init_eac_storage(ctx: Context<InitEacStorage>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    require_keys_eq!(
        ctx.accounts.producer.authority,
        signer.key(),
        CustomError::Unauthorized
    );
    let eac = &mut ctx.accounts.eac;
    eac.burned_kwts = 0;
    eac.available_kwts = 0;
    eac.available_hydrogen = 0;
    eac.producer_pubkey = ctx.accounts.producer.authority;
    Ok(())
}

#[derive(Accounts)]
pub struct InitEacStorage<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"eac", producer.authority.as_ref()],
        bump,
        space = 8 + 32 + 8 + 8 + 8
    )]
    pub eac: Account<'info, EAC>,
    pub producer: Account<'info, Producer>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
