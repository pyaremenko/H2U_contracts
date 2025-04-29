use anchor_lang::prelude::*;

use crate::{
    errors::producer::producer_errors::CustomError, state::eac::eac::EAC,
    state::h2_canister::h2_canister::H2Canister, state::producer::Producer,
};

pub fn init_h2_canister(ctx: Context<InitH2Canister>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    // Ensure the signer is the producer who owns the EAC
    require_keys_eq!(
        ctx.accounts.producer.authority,
        signer.key(),
        CustomError::Unauthorized
    );
    let canister = &mut ctx.accounts.h2_canister;
    canister.total_amount = 0;
    canister.available_amount = 0;
    canister.producer_pubkey = ctx.accounts.producer.authority;
    Ok(())
}

#[derive(Accounts)]
pub struct InitH2Canister<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"h2_canister", producer.authority.as_ref()],
        bump,
        space = 8 + 32 + 8 + 8
    )]
    pub h2_canister: Account<'info, H2Canister>,
    pub producer: Account<'info, Producer>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
