use crate::{errors::producer::producer_errors::ErrorCode, state::producer::Producer};
use anchor_lang::prelude::*;

pub fn init_producer(ctx: Context<InitProducer>, id: u64, name: String) -> Result<()> {
    let producer = &mut ctx.accounts.producer;
    if name.len() > 64 {
        return Err(ErrorCode::NameTooLong.into());
    }
    producer.id = id;
    msg!("Name length: {}", name.len());
    producer.name = name;
    producer.authority = ctx.accounts.authority.key();
    // Create token mint if not initialized

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64, name: String)]
pub struct InitProducer<'info> {
    #[account(
        init,
        seeds = [b"producer", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 8 + 4 + 64 + 32,
    )]
    pub producer: Account<'info, Producer>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
