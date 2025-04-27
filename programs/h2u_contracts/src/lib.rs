use anchor_lang::prelude::*;

declare_id!("53gF64GULwxev9mEbEL5MGbo6tJjtdtJ4U9mFb2otJQh");

#[program]
pub mod hydrogen {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
