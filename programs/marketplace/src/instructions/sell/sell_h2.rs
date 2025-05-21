use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::config::MarketConfig;
use crate::state::listing::Listing;

#[error_code]
pub enum ErrorCode {
    #[msg("The offered price is lower than the listing price.")]
    PriceTooLow,

    #[msg("Math overflow occurred during calculation.")]
    MathOverflow,
}

pub fn sell_h2(ctx: Context<SellH2>, amount: u64, offered_price: u64) -> Result<()> {
    let listing = &ctx.accounts.listing;

    require!(offered_price >= listing.price, ErrorCode::PriceTooLow);

    let total_payment = offered_price
        .checked_mul(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // Transfer SOL from buyer to producer
    // system_program::transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         system_program::Transfer {
    //             from: ctx.accounts.buyer.to_account_info(),
    //             to: ctx.accounts.producer.to_account_info(),
    //         },
    //     ),
    //     total_payment,
    // )?;

    // Transfer USDC from buyer_usdc_ata to producer_usdc_ata
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_usdc_ata.to_account_info(),
                to: ctx.accounts.producer_usdc_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        total_payment
            .checked_mul(10u64.pow(6))
            .ok_or(ErrorCode::MathOverflow)?, // USDC has 6 decimals
    )?;

    // Transfer `amount` H2 tokens from transfer_manager_ata to buyer_ata
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.transfer_manager_ata.to_account_info(),
                to: ctx.accounts.buyer_ata.to_account_info(),
                authority: ctx.accounts.transfer_manager.to_account_info(),
            },
        )
        .with_signer(&[&[
            b"transfer_manager",
            &[ctx.accounts.config.transfer_manager_bump],
        ]]),
        amount
            .checked_mul(10u64.pow(9))
            .ok_or(ErrorCode::MathOverflow)?,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct SellH2<'info> {
    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(
        mut,
        seeds = [b"listing", listing.h2_canister.as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: Buyer pays SOL
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Transfer manager PDA
    #[account(
        seeds = [b"transfer_manager"],
        bump = config.transfer_manager_bump
    )]
    pub transfer_manager: UncheckedAccount<'info>,

    #[account(mut)]
    pub transfer_manager_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_ata: Account<'info, TokenAccount>,

    /// CHECK: Recipient of SOL (producer)
    #[account(mut)]
    pub producer: UncheckedAccount<'info>,

    #[account(mut)]
    pub buyer_usdc_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub producer_usdc_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
