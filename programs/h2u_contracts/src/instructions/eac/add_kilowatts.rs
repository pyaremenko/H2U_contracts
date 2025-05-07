use anchor_lang::prelude::*;

use crate::{errors::producer::producer_errors::CustomError, state::eac::eac::EAC};

const ELECTRICITY_PER_KG_H2: u64 = 60; // 60 kWh to produce 1 kg H2
const GRAMS_PER_KG: u64 = 1000; // 1 kg = 1000 grams

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
    let minted_grams = (burned_kwh * GRAMS_PER_KG) / ELECTRICITY_PER_KG_H2;

    eac.available_kwts = eac.available_kwts.checked_add(burned_kwh).unwrap();
    eac.available_hydrogen = eac.available_hydrogen.checked_add(minted_grams).unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct AddKilowattsEac<'info> {
    #[account(mut)]
    pub eac: Account<'info, EAC>,
    #[account(mut)]
    pub authority: Signer<'info>, // whoever is authorized to update
}
