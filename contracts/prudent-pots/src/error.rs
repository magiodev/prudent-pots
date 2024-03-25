use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidFunds")]
    InvalidFunds {},

    #[error("InvalidInput")]
    InvalidInput {},

    #[error("NoFunds")]
    InsufficientFunds {},

    #[error("GameStillActive")]
    GameStillActive {},

    #[error("BidOutOfRange. Min: {min:?}, Max: {max:?}")]
    BidOutOfRange { min: Uint128, max: Uint128 },
    // #[error("Custom Error val: {val:?}")]
    // CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
