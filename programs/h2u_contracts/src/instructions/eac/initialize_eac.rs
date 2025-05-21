use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{Mint, MintTo, Token, TokenAccount},
};

use crate::state::{eac::eac::EAC, producer::Producer};

pub fn init_eac_storage(
    ctx: Context<InitEacStorage>,
    id: String,
    token_name: String,
    token_symbol: String,
    token_uri: String,
    total_amount: u64,
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

    anchor_spl::token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.producer_ata.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        total_amount * 10u64.pow(9), // use .pow for exponentiation and suffix for type
    )?;

    let eac = &mut ctx.accounts.eac;
    eac.certificate_capacity_kwts = total_amount;
    eac.available_kwts = total_amount;
    eac.burned_kwts = 0;
    eac.producer_pubkey = ctx.accounts.producer.key();
    eac.token_mint = ctx.accounts.token_mint.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct InitEacStorage<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"eac", producer.key().as_ref(), id.as_ref()],
        bump,
        space = EAC::MAXLEN
    )]
    pub eac: Account<'info, EAC>,

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
