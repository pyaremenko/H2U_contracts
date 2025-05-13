use anchor_lang::prelude::*;

use crate::{
    errors::eac::CustomError,
    state::{eac::eac::EAC, producer::Producer},
};

const ELECTRICITY_PER_KG_H2: u64 = 60; // 60 kWh to produce 1 kg H2
const GRAMS_PER_KG: u64 = 1000; // 1 kg = 1000 grams

pub fn subtract_kilowatts_eac(ctx: Context<SubtractKilowattsEac>, burned_kwh: u64) -> Result<()> {
    let eac = &mut ctx.accounts.eac;
    require!(
        eac.available_kwts >= burned_kwh,
        CustomError::NotEnoughElectricity
    );
    let minted_grams = (burned_kwh * GRAMS_PER_KG) / ELECTRICITY_PER_KG_H2;

    eac.available_kwts = eac.available_kwts.checked_sub(burned_kwh).unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct SubtractKilowattsEac<'info> {
    #[account(mut)]
    pub eac: Account<'info, EAC>,
    pub authority: Signer<'info>,
}
