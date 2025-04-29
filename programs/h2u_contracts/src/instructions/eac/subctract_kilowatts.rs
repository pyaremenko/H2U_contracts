use anchor_lang::prelude::*;

use crate::{
    errors::eac::CustomError,
    state::{eac::eac::EAC, producer::Producer},
};

#[derive(Accounts)]
pub struct SubtractKilowattsEac<'info> {
    #[account(mut)]
    pub eac: Account<'info, EAC>,
    pub authority: Signer<'info>,
}

pub fn subtract_kilowatts_eac(ctx: Context<SubtractKilowattsEac>, amount: u64) -> Result<()> {
    let eac = &mut ctx.accounts.eac;
    require!(
        eac.available_amount >= amount,
        CustomError::NotEnoughElectricity
    );
    eac.available_amount = eac.available_amount.checked_sub(amount).unwrap();
    Ok(())
}
