use anchor_lang::prelude::*;

use crate::{errors::producer::producer_errors::CustomError, state::eac::eac::EAC};

pub fn add_kilowatts_eac(ctx: Context<AddKilowattsEac>, burned_kwh: u64) -> Result<()> {
    let eac = &mut ctx.accounts.eac;
    //msg!("EAC account: {}", ctx.accounts.eac.key());
    msg!("Producer pubkey: {}", eac.producer_pubkey);
    msg!("Authority: {}", ctx.accounts.authority.key());
    require_keys_eq!(
        eac.producer_pubkey,
        ctx.accounts.authority.key(),
        CustomError::Unauthorized
    );

    eac.available_kwts = eac.available_kwts.checked_add(burned_kwh).unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct AddKilowattsEac<'info> {
    #[account(mut)]
    pub eac: Account<'info, EAC>,
    #[account(mut)]
    pub authority: Signer<'info>, // whoever is authorized to update
}
