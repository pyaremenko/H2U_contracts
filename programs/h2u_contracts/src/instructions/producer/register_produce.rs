use anchor_lang::prelude::*;

use crate::{
    instructions::{eac::burn_eac, h2_canister::mint_h2},
    state::{eac::eac::EAC, h2_canister::h2_canister::H2Canister, producer::Producer},
};

const ELECTRICITY_PER_TON_H2: u64 = 60; // 60 kWh to produce 1 ton H2

pub fn register_produceee(ctx: Context<RegisterProduce>, burned_kwh: u64) -> Result<()> {
    burn_eac(&mut ctx.accounts.eac, burned_kwh)?;
    // calculate how many tons of H2 to mint
    let minted_tons = burned_kwh / ELECTRICITY_PER_TON_H2;
    // mint hydrogen
    mint_h2(&mut ctx.accounts.h2_canister, minted_tons)?;
    Ok(())
}

pub fn register_produce(ctx: Context<RegisterProduce>, burned_kwh: u64) -> Result<()> {
    burn_eac(&mut ctx.accounts.eac, burned_kwh)?;
    let minted_tons = burned_kwh / ELECTRICITY_PER_TON_H2;
    mint_h2(&mut ctx.accounts.h2_canister, minted_tons)?;
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterProduce<'info> {
    #[account(mut, seeds = [b"producer", authority.key().as_ref()], bump)]
    pub producer: Account<'info, Producer>,

    #[account(mut, seeds = [b"h2_canister", producer.key().as_ref()], bump)]
    pub h2_canister: Account<'info, H2Canister>,

    #[account(mut, seeds = [b"eac", producer.key().as_ref()], bump)]
    pub eac: Account<'info, EAC>,

    #[account(mut)]
    pub authority: Signer<'info>,
}
