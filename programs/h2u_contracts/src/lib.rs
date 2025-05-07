use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("GeFoFS8vqoYQwcoRqLDxeg4TqEFxf3vMCGHMPV8u9rr5");

#[program]
pub mod hydrogen {

    use super::*;
    pub fn initialize_producer(ctx: Context<InitProducer>, id: u64, name: String) -> Result<()> {
        init_producer(ctx, id, name)
    }

    pub fn update_producer_data(ctx: Context<UpdateProducer>, name: String) -> Result<()> {
        update_producer(ctx, name)
    }

    pub fn initialize_eac_storage(ctx: Context<InitEacStorage>) -> Result<()> {
        init_eac_storage(ctx)
    }

    pub fn add_kilowatts_to_eac(ctx: Context<AddKilowattsEac>, amount: u64) -> Result<()> {
        add_kilowatts_eac(ctx, amount)
    }

    pub fn substract_kilowatts_from_eac(
        ctx: Context<SubtractKilowattsEac>,
        amount: u64,
    ) -> Result<()> {
        subtract_kilowatts_eac(ctx, amount)
    }

    pub fn initialize_h2_canister(ctx: Context<InitH2Canister>) -> Result<()> {
        init_h2_canister(ctx)
    }

    // pub fn burn_eac_certificate(ctx: Context<BurnEac>, burned_kwh: u64) -> Result<()> {
    //     burn_eac(ctx, , burned_kwh)
    // }

    // pub fn mint_h2_nft(ctx: Context<MintH2>, minted_tons: u64) -> Result<()> {
    //     mint_h2_context(ctx, minted_tons)
    // }

    pub fn producer_register_batch(
        ctx: Context<RegisterProduce>,
        burned_kwh: u64,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        register_produce(ctx, burned_kwh, token_name, token_symbol, token_uri)
    }

    // pub fn producer_register_batch(ctx: Context<RegisterProduce>, burned_kwh: u64) -> Result<()> {
    //     register_produce(ctx, burned_kwh)
    // }
}

#[derive(Accounts)]
pub struct Initialize {}
