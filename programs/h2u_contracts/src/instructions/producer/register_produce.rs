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
const GRAMS_PER_KG: u64 = 1000; // 1 kg = 1000 grams

pub fn register_produce(
    ctx: Context<RegisterProduce>,
    burned_kwh: u64,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Result<()> {
    let h2_canister = &mut ctx.accounts.h2_canister;

    // Create token mint if not initialized
    if h2_canister.token_mint == Pubkey::default() {
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    mint_authority: ctx.accounts.authority.to_account_info(),
                    update_authority: ctx.accounts.authority.to_account_info(),
                    payer: ctx.accounts.authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            DataV2 {
                name: token_name,
                symbol: token_symbol,
                uri: token_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            false,
            true,
            None,
        )?;
        h2_canister.token_mint = ctx.accounts.token_mint.key();
    }

    let minted_grams = (burned_kwh * GRAMS_PER_KG) / ELECTRICITY_PER_KG_H2;

    burn_eac(
        &mut ctx.accounts.eac,
        &ctx.accounts.authority,
        &ctx.accounts.token_mint,
        &ctx.accounts.producer_ata,
        &ctx.accounts.token_program,
        burned_kwh,
        minted_grams,
    )?;
    mint_h2(
        &mut ctx.accounts.h2_canister,
        &mut ctx.accounts.eac,
        &ctx.accounts.token_mint,
        &ctx.accounts.producer_ata,
        &ctx.accounts.token_program,
        &ctx.accounts.authority,
        minted_grams,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterProduce<'info> {
    #[account(mut, seeds = [b"producer", authority.key().as_ref()], bump)]
    pub producer: Account<'info, Producer>,

    #[account(mut, seeds = [b"h2_canister", authority.key().as_ref()], bump)]
    pub h2_canister: Account<'info, H2Canister>,

    #[account(mut, seeds = [b"eac", authority.key().as_ref()], bump)]
    pub eac: Account<'info, EAC>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), token_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = authority,
    )]
    pub producer_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    // CHECK: Validated as the program ID for CPI
    //pub program: AccountInfo<'info>,
}
