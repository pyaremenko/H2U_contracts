use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Not enough available electricity to burn.")]
    NotEnoughElectricity,
    #[msg("Burned electricity is not enough to produce any hydrogen.")]
    NotEnoughToProduceHydrogen,
}
