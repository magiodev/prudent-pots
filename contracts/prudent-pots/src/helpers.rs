use cosmwasm_std::{Addr, Deps, DepsMut, Env, QuerierWrapper, StdError, StdResult, Uint128};

use crate::{
    state::{PotState, PLAYER_ALLOCATIONS, POT_STATES},
    ContractError,
};

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

// Helper to determine if a pot is a winning pot based on its unique rules
pub fn is_winning_pot(deps: &Deps, pot_id: u8) -> StdResult<bool> {
    let pot_state = POT_STATES.load(deps.storage, pot_id)?;
    let total_tokens = pot_state.total_tokens;

    match pot_id {
        1 => {
            // For Median Pot: Compare with other pots to determine if it's the median
            let token_counts = get_all_token_counts(deps)?;
            Ok(is_median(&token_counts, total_tokens))
        }
        2 => {
            // For Highest Pot: Compare with other pots to determine if it's the highest
            let max_tokens = get_max_tokens(deps)?;
            Ok(total_tokens == max_tokens)
        }
        3 => {
            // For Even Pot: Check if the token count is even
            Ok(total_tokens % Uint128::from(2u128) == Uint128::zero())
        }
        4 => {
            // For Lowest Pot: Compare with other pots to determine if it's the lowest
            let min_tokens = get_min_tokens(deps)?;
            Ok(total_tokens == min_tokens)
        }
        5 => {
            // For Prime Pot: Check if the token count is a prime number
            Ok(is_prime(total_tokens.u128()))
        }
        _ => Err(StdError::generic_err("Invalid pot ID")),
    }
}

// Retrieve the token count for each pot
fn get_all_token_counts(deps: &Deps) -> StdResult<Vec<Uint128>> {
    let mut token_counts = Vec::new();
    for pot_id in 1..=5 {
        // Assuming 5 pots
        let pot_state = POT_STATES.load(deps.storage, pot_id)?;
        token_counts.push(pot_state.total_tokens);
    }
    Ok(token_counts)
}

// Check if a value is the median in a vector of token counts
fn is_median(token_counts: &Vec<Uint128>, value: Uint128) -> bool {
    let mut sorted_counts = token_counts.clone();
    sorted_counts.sort_unstable();
    let mid = sorted_counts.len() / 2;

    if sorted_counts.len() % 2 == 0 {
        (sorted_counts[mid - 1] <= value) && (value <= sorted_counts[mid])
    } else {
        value == sorted_counts[mid]
    }
}

// Get the maximum token count from all pots
fn get_max_tokens(deps: &Deps) -> StdResult<Uint128> {
    let token_counts = get_all_token_counts(deps)?;
    Ok(*token_counts.iter().max().unwrap_or(&Uint128::zero()))
}

// Get the minimum token count from all pots
fn get_min_tokens(deps: &Deps) -> StdResult<Uint128> {
    let token_counts = get_all_token_counts(deps)?;
    Ok(*token_counts.iter().min().unwrap_or(&Uint128::zero()))
}

// Check if a number is prime
fn is_prime(number: u128) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=(number as f64).sqrt() as u128 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

// Helper to calculate the total tokens in losing pots
pub fn calculate_total_losing_tokens(deps: &Deps, winning_pots: &[u8]) -> StdResult<Uint128> {
    let mut total_losing_tokens = Uint128::zero();
    for pot_id in 1..=5 {
        // Assuming 5 pots
        if !winning_pots.contains(&pot_id) {
            let pot_state = POT_STATES.load(deps.storage, pot_id)?;
            total_losing_tokens += pot_state.total_tokens;
        }
    }
    Ok(total_losing_tokens)
}

// Helper to redistribute losing tokens
pub fn redistribute_losing_tokens(
    deps: DepsMut,
    winning_pots: &[u8],
    total_losing_tokens: Uint128,
) -> StdResult<()> {
    // Calculate the amount to redistribute to the winning pots (50% of the losing tokens)
    let redistribution = total_losing_tokens.multiply_ratio(Uint128::new(1), Uint128::new(2));

    for pot_id in 1..=5 {
        // Assuming 5 pots
        if !winning_pots.contains(&pot_id) {
            continue; // Skip non-winning pots
        }

        let pot_state = POT_STATES.load(deps.storage, pot_id)?;
        let total_pot_tokens = pot_state.total_tokens;
        let mut total_redistributed = Uint128::zero();

        // Collect updates first to avoid borrowing issues
        let mut allocation_updates = vec![];

        // Determine each player's share and collect the updates
        let player_allocations = PLAYER_ALLOCATIONS
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| item.ok());

        for (addr, mut allocations) in player_allocations {
            if let Some(allocation) = allocations
                .allocations
                .iter_mut()
                .find(|a| a.pot_id == pot_id)
            {
                let player_share =
                    redistribution.multiply_ratio(allocation.amount, total_pot_tokens);
                allocation.amount = allocation.amount.checked_add(player_share)?;
                total_redistributed += player_share;
                allocation_updates.push((addr, allocations.clone()));
            }
        }

        // Apply the collected updates
        for (addr, allocations) in allocation_updates {
            PLAYER_ALLOCATIONS.save(deps.storage, addr, &allocations)?;
        }

        // Update the pot's state with the total redistributed amount
        POT_STATES.update(deps.storage, pot_id, |pot_state| -> StdResult<_> {
            let state = pot_state.unwrap();
            Ok(PotState {
                total_tokens: state.total_tokens.checked_add(total_redistributed)?,
            })
        })?;
    }

    Ok(())
}

// Helper to prepare for the next game
pub fn prepare_next_game(deps: &Deps, redistribution: Uint128) -> StdResult<()> {
    // Placeholder logic: Prepare the game state and pots for the next game
    // This might involve resetting pot states or setting initial tokens for the next game
    Ok(())
}

// /// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
// /// for working with this.
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// pub struct CwTemplateContract(pub Addr);

// impl CwTemplateContract {
//     pub fn addr(&self) -> Addr {
//         self.0.clone()
//     }

//     pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
//         let msg = to_json_binary(&msg.into())?;
//         Ok(WasmMsg::Execute {
//             contract_addr: self.addr().into(),
//             msg,
//             funds: vec![],
//         }
//         .into())
//     }

//     // /// Get Count
//     // pub fn count<Q, T, CQ>(&self, querier: &Q) -> StdResult<GetCountResponse>
//     // where
//     //     Q: Querier,
//     //     T: Into<String>,
//     //     CQ: CustomQuery,
//     // {
//     //     let msg = QueryMsg::GetCount {};
//     //     let query = WasmQuery::Smart {
//     //         contract_addr: self.addr().into(),
//     //         msg: to_json_binary(&msg)?,
//     //     }
//     //     .into();
//     //     let res: GetCountResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
//     //     Ok(res)
//     // }
// }
