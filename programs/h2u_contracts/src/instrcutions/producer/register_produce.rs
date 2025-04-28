use anchor_lang::prelude::*;

pub fn register_produce(ctx: Context<RegisterProduce>, burned_kwh: u64) -> Result<()> {
    burn_eac(&mut ctx.accounts.eac, burned_kwh)?;
    mint_h2(&mut ctx.accounts.h2_canister, burned_kwh)?;
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterProduce<'info> {
    #[account(mut, seeds = [b"producer", authority.key().as_ref()], bump)]
    pub producer: Account<'info, Producer>,

    #[account(mut, seeds = [b"h2_canister", producer.key().as_ref()], bump)]
    pub h2_canister: Account<'info, H2Canister>,

    #[account(mut, seeds = [b"eac", producer.key().as_ref()], bump)]
    pub eac: Account<'info, EAC>,

    #[account(mut)]
    pub authority: Signer<'info>,
}
