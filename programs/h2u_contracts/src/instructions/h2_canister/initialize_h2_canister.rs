use anchor_lang::prelude::*;

use crate::{
    errors::h2_canister::CustomError,
    state::{h2_canister::h2_canister::H2Canister, producer::Producer},
};

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{Mint, Token, TokenAccount},
};

pub fn init_h2_canister(
    ctx: Context<InitH2Canister>,
    id: String,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Result<()> {
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
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

    // Ensure the signer is the producer who owns the EAC (optional enforcement)
    // require_keys_eq!(
    //     ctx.accounts.producer.authority,
    //     signer.key(),
    //     CustomError::Unauthorized
    // );

    let canister = &mut ctx.accounts.h2_canister;
    canister.batch_id = id;
    canister.total_amount = 0;
    canister.available_hydrogen = 0;
    canister.producer_pubkey = ctx.accounts.producer.authority;
    canister.token_mint = ctx.accounts.token_mint.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct InitH2Canister<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"h2_canister", producer.authority.as_ref(), id.as_ref()],
        bump,
        space = 8 + 32 + 8 + 8 + 32 + 4 + 36, // discriminator + total_amount + available_hydrogen + producer_pubkey + token_mint + batch_id (4 + 36)
        constraint = id.len() <= 36 @ CustomError::BatchIdTooLong
    )]
    pub h2_canister: Account<'info, H2Canister>,

    #[account(mut)]
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
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = signer,
    )]
    pub producer_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub producer: Account<'info, Producer>,
    pub rent: Sysvar<'info, Rent>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
