use anchor_lang::prelude::*;

use crate::{
    errors::eac::CustomError,
    state::{eac::eac::EAC, h2_canister::h2_canister::H2Canister, producer::Producer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

pub fn mint_h2<'info>(
    h2_canister: &mut Account<'info, H2Canister>,
    token_mint: &Account<'info, Mint>,
    producer_ata: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    authority: &Signer<'info>,
    minted_grams: u64,
) -> Result<()> {
    // require!(
    //     eac.available_hydrogen >= minted_grams,
    //     CustomError::NotEnoughToProduceHydrogen
    // );

    // mint H2 tokens
    mint_to(
        CpiContext::new(
            token_program.to_account_info(),
            MintTo {
                mint: token_mint.to_account_info(),
                to: producer_ata.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        minted_grams * 10u64.pow(token_mint.decimals as u32),
    )?;

    h2_canister.total_amount = h2_canister.total_amount.checked_add(minted_grams).unwrap();
    h2_canister.available_hydrogen = h2_canister
        .available_hydrogen
        .checked_add(minted_grams)
        .unwrap();
    Ok(())
}

// pub fn mint_h2_context(ctx: Context<MintH2>, burned_kwh: u64 , minted_grams: u64) -> Result<()> {
//     mint_h2(&mut ctx.accounts.h2_canister, , , burned_kwh, minted_grams)
// }

// #[derive(Accounts)]
// pub struct MintH2<'info> {
//     #[account(mut, seeds = [b"h2_canister", producer.key().as_ref()], bump)]
//     pub h2_canister: Account<'info, H2Canister>,

//     pub producer: Account<'info, Producer>,

//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(mut)]
//     pub token_mint: Account<'info, Mint>,
//     #[account(
//             mut,
//             associated_token::mint = token_mint,
//             associated_token::authority = authority,
//         )]
//     pub producer_ata: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
//     /// CHECK: Validated as the program ID for CPI
//     pub program: AccountInfo<'info>,
// }
