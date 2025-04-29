use anchor_lang::prelude::*;

use crate::{
    errors::eac::CustomError,
    state::{eac::eac::EAC, producer::Producer},
};

pub fn burn_eac(eac: &mut Account<EAC>, burned_kwh: u64) -> Result<()> {
    require!(
        eac.available_amount >= burned_kwh,
        CustomError::NotEnoughElectricity
    );
    eac.available_amount = eac.available_amount.checked_sub(burned_kwh).unwrap();
    eac.burned_amount = eac.burned_amount.checked_add(burned_kwh).unwrap();
    Ok(())
}

pub fn burn_eac_with_context(ctx: Context<BurnEac>, burned_kwh: u64) -> Result<()> {
    burn_eac(&mut ctx.accounts.eac, burned_kwh)
}

#[derive(Accounts)]
pub struct BurnEac<'info> {
    #[account(mut, seeds = [b"eac", producer.key().as_ref()], bump)]
    pub eac: Account<'info, EAC>,

    pub producer: Account<'info, Producer>,
}
