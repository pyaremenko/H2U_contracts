use anchor_lang::prelude::*;

use crate::state::{h2_canister::h2_canister::H2Canister, producer::Producer};

pub fn mint_h2(h2_canister: &mut Account<H2Canister>, minted_tons: u64) -> Result<()> {
    h2_canister.total_amount = h2_canister.total_amount.checked_add(minted_tons).unwrap();
    h2_canister.available_amount = h2_canister
        .available_amount
        .checked_add(minted_tons)
        .unwrap();
    Ok(())
}

pub fn mint_h2_context(ctx: Context<MintH2>, burned_kwh: u64) -> Result<()> {
    mint_h2(&mut ctx.accounts.h2_canister, burned_kwh)
}

#[derive(Accounts)]
pub struct MintH2<'info> {
    #[account(mut, seeds = [b"h2_canister", producer.key().as_ref()], bump)]
    pub h2_canister: Account<'info, H2Canister>,

    pub producer: Account<'info, Producer>,
}
