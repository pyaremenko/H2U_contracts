use anchor_lang::prelude::*;

use crate::{errors::producer::producer_errors::ErrorCode, state::producer::Producer};

pub fn update_producer(ctx: Context<UpdateProducer>, name: String) -> Result<()> {
    let producer = &mut ctx.accounts.producer;
    msg!("Name length: {}", name.len());
    if name.len() > 64 {
        return Err(ErrorCode::NameTooLong.into());
    }
    producer.name = name;
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateProducer<'info> {
    #[account(mut, seeds = [b"producer", authority.key().as_ref()], bump)]
    pub producer: Account<'info, Producer>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
