use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitEac<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"eac", producer.key().as_ref()],
        bump,
        space = 8 + 32 + 8 + 8 + 8
    )]
    pub eac: Account<'info, EAC>,

    pub producer: Account<'info, Producer>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_eac(ctx: Context<InitEac>) -> Result<()> {
    let eac = &mut ctx.accounts.eac;
    eac.total_amount = 0;
    eac.available_amount = 0;
    eac.producer_pubkey = ctx.accounts.producer.key();
    Ok(())
}