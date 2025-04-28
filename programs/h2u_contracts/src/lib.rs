use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("53gF64GULwxev9mEbEL5MGbo6tJjtdtJ4U9mFb2otJQh");

#[program]
pub mod hydrogen {

    use super::*;
    pub fn initialize_producer(ctx: Context<InitProducer>, id: u64, name: String) -> Result<()> {
        init_producer(ctx, id, name)
    }
    pub fn initialize_eac_storage(ctx: Context<InitEacStorage>) -> Result<()> {
        init_eac_storage(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
