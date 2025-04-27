use anchor_lang::prelude::*;

declare_id!("53CP3kVEC2YnSd3ionqHRNeJ9ZheeBgghogRjPtkcjPr");

#[program]
pub mod h2u_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
