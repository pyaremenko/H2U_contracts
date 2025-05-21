use crate::{
    errors::{eac::CustomError, ErrorMOR},
    state::{eac::eac::EAC, h2_canister::h2_canister::H2Canister, producer::Producer},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn as TokenBurn, Mint, Token, TokenAccount};

pub fn burn_eac<'info>(
    eac: &mut Account<'info, EAC>,
    authority: &Signer<'info>,
    token_mint: &Account<'info, Mint>,
    producer_ata: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    burned_kwh: u64,
    burned_grams: u64, // for state tracking only
) -> Result<()> {
    msg!("Starting burn_eac...");
    msg!(
        "Requested burn: {} kWh ({} grams H2 equivalent)",
        burned_kwh,
        burned_grams
    );

    msg!("Current available: {} kWh", eac.available_kwts);
    msg!("Current burned: {} kWh", eac.burned_kwts);

    // You probably want to burn exactly the number of kWhs as tokens, if 1 token = 1 kWh
    let token_amount_to_burn = burned_kwh;

    require!(
        eac.available_kwts >= burned_kwh,
        CustomError::NotEnoughElectricity
    );

    require!(
        producer_ata.amount >= token_amount_to_burn,
        ErrorMOR::InvalidBurnAmount
    );

    // Burn tokens from producer ATA
    token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            TokenBurn {
                mint: token_mint.to_account_info(),
                from: producer_ata.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        token_amount_to_burn * 10u64.pow(token_mint.decimals as u32),
    )?;

    eac.available_kwts = eac.available_kwts.checked_sub(burned_kwh).unwrap();
    eac.burned_kwts = eac.burned_kwts.checked_add(burned_kwh).unwrap();

    msg!("✅ Burn complete. Updated EAC:");
    msg!(" - Remaining: {} kWh", eac.available_kwts);
    msg!(" - Total burned: {} kWh", eac.burned_kwts);

    Ok(())
}

#[derive(Accounts)]
pub struct BurnEac<'info> {
    #[account(mut, seeds = [b"eac", producer.key().as_ref()], bump)]
    pub eac: Account<'info, EAC>,

    #[account(
            mut,
            seeds = [b"h2_canister", producer.key().as_ref()],
            bump
        )]
    pub h2_canister: Account<'info, H2Canister>,

    #[account(seeds = [b"producer", authority.key().as_ref()], bump)]
    pub producer: Account<'info, Producer>,

    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(
            mut,
            associated_token::mint = token_mint,
            associated_token::authority = authority,
        )]
    pub producer_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // CHECK: Validated as the program ID for CPI
    pub program: AccountInfo<'info>,
}
