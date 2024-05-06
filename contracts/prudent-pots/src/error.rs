use cosmwasm_std::{DivideByZeroError, OverflowError, StdError, Uint128};
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

    #[error("InvalidPot")]
    InvalidPot {},

    #[error("AlreadyAllocated")]
    AlreadyAllocated {},

    #[error("NoFunds")]
    InsufficientFunds {},

    #[error("GameAlreadyEnded")]
    GameAlreadyEnded {},

    #[error("GameStillActive")]
    GameStillActive {},

    #[error("PotLimitReached: A pot cannot contain more tokens than the sum of the others.")]
    PotLimitReached {},

    #[error("PreviousRaffleNftIsUnwon")]
    PreviousRaffleNftIsUnwon {},

    #[error("Cw721TokenNotReceived")]
    Cw721TokenNotReceived {},

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("BidOutOfRange. Min: {min:?}, Max: {max:?}")]
    BidOutOfRange { min: Uint128, max: Uint128 },

    #[error("NotEnoughFundsForNextRound")]
    NotEnoughFundsForNextRound {},

    #[error("Unknown Reply ID")]
    UnknownReply {},
    // #[error("Custom Error val: {val:?}")]
    // CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
