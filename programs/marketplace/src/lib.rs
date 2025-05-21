use anchor_lang::prelude::*;

declare_id!("2tXBiHkHxsPFbh7RjJwxdZC2eFupBATaZ1GdE1ngadaM");

pub mod instructions;
pub mod state;

pub use instructions::*;
#[program]
pub mod marketplace {

    use super::*;

    pub fn initialize_config(ctx: Context<InitConfig>) -> Result<()> {
        init_config(ctx)
    }

    pub fn update_configuration(ctx: Context<UpdateConfig>) -> Result<()> {
        update_config(ctx)
    }

    pub fn list_tokens(ctx: Context<ListH2>, amount: u64, price: u64) -> Result<()> {
        list_h2(ctx, amount, price)
    }

    pub fn sell_tokens(ctx: Context<SellH2>, amount: u64, offered_price: u64) -> Result<()> {
        sell_h2(ctx, amount, offered_price)
    }
}
