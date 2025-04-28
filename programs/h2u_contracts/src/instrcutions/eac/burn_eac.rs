use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BurnEac<'info> {
    #[account(mut, seeds = [b"eac", producer.key().as_ref()], bump)]
    pub eac: Account<'info, EAC>,
    
    pub producer: Account<'info, Producer>,
}

pub fn burn_eac(ctx: Context<BurnEac>, amount: u64) -> Result<()> {
    let eac = &mut ctx.accounts.eac;
    require!(eac.available_amount >= amount, CustomError::NotEnoughBalance);
    eac.available_amount = eac.available_amount.checked_sub(amount).unwrap();
    Ok(())
}