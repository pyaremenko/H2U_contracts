use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("5eXrKzvXYjfyWAVgYjngphm6rF4LpFkHVUBDqWbzPMg3");

#[program]
pub mod oracle {
    use super::*;

    pub fn init_config(ctx: Context<InitializeOracleConfig>) -> Result<()> {
        initialize_oracle_config(ctx)
    }

    pub fn update_price(ctx: Context<UpdateOraclePrice>, new_min: u64, new_max: u64) -> Result<()> {
        update_oracle_price(ctx, new_min, new_max)
    }

    pub fn update_coinfig(ctx: Context<UpdateOracleConfig>, new_admin: Pubkey) -> Result<()> {
        update_oracle_config(ctx, new_admin)
    }
}
