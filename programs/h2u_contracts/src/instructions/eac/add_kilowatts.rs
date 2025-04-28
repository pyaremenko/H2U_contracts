use anchor_lang::prelude::*;

use crate::{
    errors::eac::eac_errors::CustomError,
    state::{eac::eac::EAC, producer::Producer},
};

#[derive(Accounts)]
pub struct AddKilowattsEac<'info> {
    #[account(mut)]
    pub eac: Account<'info, EAC>,
    pub authority: Signer<'info>, // whoever is authorized to update
}

pub fn add_kilowatts_eac(ctx: Context<AddKilowattsEac>, amount: u64) -> Result<()> {
    let eac = &mut ctx.accounts.eac;
    require_keys_eq!(
        eac.producer_pubkey,
        ctx.accounts.authority.key(),
        CustomError::Unauthorized
    );
    eac.available_amount = eac.available_amount.checked_add(amount).unwrap();
    Ok(())
}
