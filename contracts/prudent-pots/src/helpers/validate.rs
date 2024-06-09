use cosmwasm_std::{Addr, Coin, Env, QuerierWrapper, Storage, Uint128};

use crate::{
    state::{GAME_CONFIG, GAME_STATE, PLAYER_ALLOCATIONS, PLAYER_REALLOCATIONS, POT_STATES},
    ContractError,
};

// This is meant to give Unauthorized even after the Admin ownership over the contract has been thrown.
pub fn validate_is_contract_admin(
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

// That function is meant to add a permissioned time-based layer over the game_end function,
// leaving it permissionles after a threshold time if the admin doesn't.
pub fn validate_is_contract_admin_game_end(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let game_config = GAME_CONFIG.load(storage)?;
    let game_state = GAME_STATE.load(storage)?;

    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;

    // Calculate if current time is within the threshold time after game end
    if env.block.time.seconds()
        < game_state
            .end_time
            .checked_add(game_config.game_end_threshold)
            .unwrap()
    {
        if let Some(contract_admin) = contract_admin {
            if contract_admin != *sus_admin {
                return Err(ContractError::Unauthorized {});
            }
        } else {
            // Within the threshold time, if there is no contract admin, any user can execute the function regardless of admin status
        }
    } else {
        // After the threshold time, any user can execute the function regardless of admin status
    }
    Ok(())
}

pub fn validate_pot_initial_amount(
    min_pot_initial_allocation: &Uint128,
    total_amount: &Uint128,
) -> Result<Uint128, ContractError> {
    let amount_per_pot = total_amount.checked_div(Uint128::new(5u128))?;

    // Assuming there are 5 pots
    if amount_per_pot.lt(min_pot_initial_allocation) {
        Err(ContractError::NotEnoughFundsForNextRound {})
    } else {
        Ok(amount_per_pot)
    }
}

pub fn validate_increase_player_reallocations(
    storage: &mut dyn Storage,
    player: &Addr,
) -> Result<(), ContractError> {
    // Load game configuration and the current number of reallocations for the player.
    let game_config = GAME_CONFIG.load(storage)?;
    let current_reallocations = PLAYER_REALLOCATIONS
        .may_load(storage, player.to_string())?
        .unwrap_or_default();

    // Check if the player has reached the reallocation limit.
    if current_reallocations >= game_config.reallocations_limit {
        return Err(ContractError::ReallocationsLimitReached {});
    }

    // Safely increase the reallocation count, handling potential overflow.
    let new_reallocations = current_reallocations
        .checked_add(1)
        .ok_or(ContractError::InvalidInput {})?;

    // Save the updated number of reallocations.
    PLAYER_REALLOCATIONS.save(storage, player.to_string(), &new_reallocations)?;

    Ok(())
}

/// Checks if the specified player has already allocated tokens to the specified pot.
/// Returns `Ok(())` if no allocations exist or if the allocation is zero, or `Err(ContractError::AlreadyAllocated)` if a non-zero allocation is found.
pub fn validate_existing_allocation(
    storage: &dyn Storage,
    player: &Addr,
    pot_id: u8,
) -> Result<(), ContractError> {
    let player_allocs_opt = PLAYER_ALLOCATIONS.may_load(storage, player.to_string())?;

    if let Some(player_allocs) = player_allocs_opt {
        // If allocations exist, check if any non-zero allocation is made to the specified pot.
        if player_allocs
            .iter()
            .any(|alloc| alloc.pot_id == pot_id && !alloc.amount.is_zero())
        {
            return Err(ContractError::AlreadyAllocated {});
        }
    }

    Ok(())
}

pub fn validate_pot_limit_not_exceeded(
    storage: &dyn Storage,
    pot_id: u8,
    amount: Uint128,
) -> Result<(), ContractError> {
    let pots = POT_STATES.range(storage, None, None, cosmwasm_std::Order::Ascending);
    let mut sum_of_other_pots = Uint128::zero();
    let mut current_pot_amount = Uint128::zero();

    // Calculate the sum of all other pots and find the current pot amount
    for item in pots {
        let (id, token_allocation) = item?;
        if id == pot_id {
            current_pot_amount = token_allocation.amount;
        } else {
            sum_of_other_pots = sum_of_other_pots.checked_add(token_allocation.amount)?;
        }
    }

    // Calculate new pot amount
    let new_current_pot_amount = current_pot_amount.checked_add(amount)?;

    // Check if adding the new amount to the current pot exceeds the sum of all other pots
    if new_current_pot_amount > sum_of_other_pots {
        Err(ContractError::PotLimitReached {})
    } else {
        Ok(())
    }
}

// Helper to validate the game's end time and extend it if necessary during the round for allocations and reallocations
pub fn validate_and_extend_game_time(
    storage: &mut dyn Storage,
    env: &Env,
) -> Result<(), ContractError> {
    let game_config = GAME_CONFIG.load(storage)?;
    let mut game_state = GAME_STATE.load(storage)?;

    // Check if the game has already ended
    if env.block.time.seconds() >= game_state.end_time {
        return Err(ContractError::GameAlreadyEnded {});
    }

    // Calculate the remaining time
    let remaining_time = game_state
        .end_time
        .checked_sub(env.block.time.seconds())
        .unwrap();

    // Extend the game time if the remaining time is less than or equal to game_config.game_extend
    if remaining_time <= game_config.game_extend {
        game_state.end_time = env.block.time.seconds() + game_config.game_extend;
        game_state.extend_count = game_state.extend_count.checked_add(1).unwrap();
        GAME_STATE.save(storage, &game_state)?;
    }

    Ok(())
}

// Helper to validate the game's end time during game_end exeuction
pub fn validate_game_end_time(storage: &dyn Storage, env: &Env) -> Result<(), ContractError> {
    let game_state = GAME_STATE.load(storage)?;

    // Check if the game is still active
    if env.block.time.seconds().lt(&game_state.end_time) {
        return Err(ContractError::GameStillActive {});
    }

    Ok(())
}

// Helper to validate and sum the funds in the specified denomination
pub fn validate_funds(funds: &[Coin], expected_denom: &str) -> Result<Uint128, ContractError> {
    let total_amount = funds.iter().fold(Uint128::zero(), |acc, coin| {
        if coin.denom == expected_denom {
            acc.checked_add(coin.amount).unwrap()
        } else {
            acc
        }
    });

    if total_amount.is_zero() {
        return Err(ContractError::InvalidFunds {});
    }

    Ok(total_amount)
}
