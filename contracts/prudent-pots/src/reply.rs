use cosmwasm_std::{attr, Response, SubMsgResult};

use crate::ContractError;

// This reply function is called by reply_always. This should only do nothing, or throw an error, foreach NFT send during game_end workflow to the raflle winner.
pub fn game_end_reply(msg: SubMsgResult) -> Result<Response, ContractError> {
    match msg.into() {
        Ok(_data) => Ok(Response::new().add_attributes(vec![
            attr("method", "reply"),
            attr("action", "game_end_reply"),
        ])),
        _ => Err(ContractError::Cw721TokenNotReceived {}),
    }
}
