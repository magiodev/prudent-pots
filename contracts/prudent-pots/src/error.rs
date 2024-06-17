use cosmwasm_std::{DivideByZeroError, OverflowError, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Transaction contains invalid funds amount.")]
    InvalidFunds {},

    #[error("Input provided is invalid.")]
    InvalidInput {},

    #[error("Invalid pot detected.")]
    InvalidPot {},

    #[error("This pot has already been allocated some funds.")]
    AlreadyAllocated {},

    #[error("Insufficient funds to complete the transaction.")]
    InsufficientFunds {},

    #[error("The reallocations limit has been reached for your address.")]
    ReallocationsLimitReached {},

    #[error("This action cannot be performed as the game has not started yet.")]
    GameNotStarted {},

    #[error("This action cannot be performed as the game has already ended.")]
    GameAlreadyEnded {},

    #[error("The game is still active and cannot be ended or reset at this time.")]
    GameStillActive {},

    #[error("Pot limit exceeded: Cannot allocate more tokens to a pot than the collective sum of others.")]
    PotLimitReached {},

    #[error("The previous raffle NFT remains unwon.")]
    PreviousRaffleNftIsUnwon {},

    #[error("Raffle NFT specified is invalid.")]
    InvalidRaffleNft {},

    #[error("The next game start time is invalid.")]
    InvalidNextGameStart {},

    #[error("Expected a CW721 token transfer but none was received.")]
    Cw721TokenNotReceived {},

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("Bid amount out of range. Min: {min:?}, Max: {max:?}")]
    BidOutOfRange { min: Uint128, max: Uint128 },

    #[error("Insufficient funds available for initiating the next round.")]
    NotEnoughFundsForNextRound {},

    #[error("Unknown Reply ID")]
    UnknownReply {},
    // #[error("Custom Error val: {val:?}")]
    // CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
