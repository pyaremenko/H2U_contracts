use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::listing::Listing;

pub fn list_h2(ctx: Context<ListH2>, amount: u64, price: u64) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    listing.h2_canister = ctx.accounts.h2_canister.key();
    listing.producer = ctx.accounts.producer.key();
    listing.price = price;
    listing.transfer_manager_ata = ctx.accounts.transfer_manager_ata.key();

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.producer_ata.to_account_info(),
                to: ctx.accounts.transfer_manager_ata.to_account_info(),
                authority: ctx.accounts.producer_authority.to_account_info(),
            },
        ),
        amount * 10u64.pow(9 as u32),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct ListH2<'info> {
    #[account(
        init,
        payer = producer_authority,
        seeds = [b"listing", h2_canister.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 8 + 32,
    )]
    pub listing: Account<'info, Listing>,

    #[account(mut)]
    pub producer_authority: Signer<'info>,

    /// CHECK: Producer PDA (not used directly)
    pub producer: UncheckedAccount<'info>,

    /// CHECK: H2Canister PDA
    pub h2_canister: UncheckedAccount<'info>,

    #[account(mut)]
    pub producer_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub transfer_manager_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
