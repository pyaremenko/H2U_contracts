use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MintH2<'info> {
    #[account(mut, seeds = [b"h2_canister", producer.key().as_ref()], bump)]
    pub h2_canister: Account<'info, H2Canister>,
    
    pub producer: Account<'info, Producer>,
}

pub fn mint_h2(ctx: Context<MintH2>, amount: u64) -> Result<()> {
    let canister = &mut ctx.accounts.h2_canister;
    canister.total_amount = canister.total_amount.checked_add(amount).unwrap();
    canister.available_amount = canister.available_amount.checked_add(amount).unwrap();
    Ok(())
}