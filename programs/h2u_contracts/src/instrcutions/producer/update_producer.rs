use anchor_lang::prelude::*;

pub fn update_producer(ctx: Context<UpdateProducer>, id: u64, name: String) -> Result<()> {
    let producer = &mut ctx.accounts.producer;
    producer.name = name;
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64, name: String)]
pub struct UpdateProducer<'info> {
    #[account(
        init,
        seeds = [b"producer", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 8 + 4 + name.len() + 32, // 8 = account discriminator
    )]
    pub producer: Account<'info, Producer>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}