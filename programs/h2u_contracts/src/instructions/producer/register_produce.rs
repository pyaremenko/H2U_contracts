use crate::{
    instructions::{eac::burn_eac, h2_canister::mint_h2},
    state::{eac::eac::EAC, h2_canister::h2_canister::H2Canister, producer::Producer},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{Mint, Token, TokenAccount},
};

const ELECTRICITY_PER_KG_H2: u64 = 60; // 60 kWh to produce 1 kg H2

pub fn register_produce(ctx: Context<RegisterProduce>, id: String, burned_kwh: u64) -> Result<()> {
    let minted_grams = (burned_kwh) / ELECTRICITY_PER_KG_H2;

    burn_eac(
        &mut ctx.accounts.eac,
        &ctx.accounts.authority,
        &ctx.accounts.eac_mint,
        &ctx.accounts.producer_eac_ata,
        &ctx.accounts.token_program,
        burned_kwh,
        minted_grams,
    )?;
    mint_h2(
        &mut ctx.accounts.h2_canister,
        &ctx.accounts.h2_mint,
        &ctx.accounts.producer_h2_ata,
        &ctx.accounts.token_program,
        &ctx.accounts.authority,
        minted_grams,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct RegisterProduce<'info> {
    #[account(mut, seeds = [b"producer", authority.key().as_ref()], bump)]
    pub producer: Account<'info, Producer>,

    #[account(mut, seeds = [b"h2_canister", authority.key().as_ref(),id.as_ref() ], bump)]
    pub h2_canister: Account<'info, H2Canister>,

    #[account(mut)]
    pub eac: Account<'info, EAC>,

    #[account(mut)]
    pub h2_mint: Account<'info, Mint>,

    #[account(mut)]
    pub eac_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = h2_mint,
        associated_token::authority = authority,
    )]
    pub producer_h2_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = eac_mint,
        associated_token::authority = authority,
    )]
    pub producer_eac_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    // CHECK: Validated as the program ID for CPI
    //pub program: AccountInfo<'info>,
}
