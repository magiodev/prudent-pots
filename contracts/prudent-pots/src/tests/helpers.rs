use crate::state::{
    GameConfig, GameState, PlayerAllocations, TokenAllocation, GAME_CONFIG, GAME_STATE,
    PLAYER_ALLOCATIONS, POT_STATES, REALLOCATION_FEE_POOL,
};

use cosmwasm_std::{Addr, DepsMut, Env, StdError, Uint128};

// Setup fixtures

pub fn setup_pots(deps: &mut DepsMut, tokens: Vec<Uint128>) {
    for (i, &amount) in tokens.iter().enumerate() {
        let pot_id = i as u8 + 1; // Pot IDs are 1-indexed
        POT_STATES
            .save(deps.storage, pot_id, &TokenAllocation { pot_id, amount })
            .unwrap();
    }
}

pub fn setup_pots_and_allocations(
    deps: &mut DepsMut,
    pots: Vec<Uint128>,
    allocations: Vec<(u8, Addr, Uint128)>,
) {
    GAME_CONFIG
        .save(
            deps.storage,
            &GameConfig {
                fee_allocation: 2,
                fee_reallocation: 5,
                fee_allocation_address: Addr::unchecked("addy"),
                game_duration: 3600,
                game_extend: 600,
                game_denom: "token".to_string(),
                min_bid: Uint128::new(1000000u128),
            },
        )
        .unwrap();

    REALLOCATION_FEE_POOL
        .save(deps.storage, &Uint128::zero())
        .unwrap();

    for (i, &amount) in pots.iter().enumerate() {
        let pot_id = i as u8 + 1; // Pot IDs are 1-indexed
        POT_STATES
            .save(deps.storage, pot_id, &TokenAllocation { pot_id, amount })
            .unwrap();
    }

    for (pot_id, player, amount) in allocations {
        // Load existing allocations or initialize if not found
        let mut player_allocations = PLAYER_ALLOCATIONS
            .load(deps.storage, player.clone())
            .unwrap_or_else(|_| PlayerAllocations {
                allocations: vec![],
            });

        // Add new allocation
        player_allocations
            .allocations
            .push(TokenAllocation { pot_id, amount });

        // Save the updated allocations
        PLAYER_ALLOCATIONS
            .save(deps.storage, player, &player_allocations)
            .unwrap();

        // Update the pot's total tokens to reflect the player's allocation
        POT_STATES
            .update(deps.storage, pot_id, |pot_state| -> Result<_, StdError> {
                let mut state = pot_state.unwrap();
                state.amount = state.amount.checked_add(amount).unwrap();
                Ok(state)
            })
            .unwrap();
    }
}

pub fn setup_game_end(deps: &mut DepsMut, env: &mut Env) {
    let game_config = GameConfig {
        game_duration: 3600, // 1 hour
        game_extend: 600,
        fee_allocation: 2,
        fee_reallocation: 5,
        fee_allocation_address: Addr::unchecked("fee_address"),
        game_denom: "token".to_string(),
        min_bid: Uint128::new(1000000u128),
    };
    GAME_CONFIG.save(deps.storage, &game_config).unwrap();

    let game_state = GameState {
        start_time: env.block.time.seconds() - 3600, // Started 1 hour ago
        end_time: env.block.time.seconds(),          // Ends now
    };
    GAME_STATE.save(deps.storage, &game_state).unwrap();

    // Initialize the REALLOCATION_FEE_POOL with zero
    REALLOCATION_FEE_POOL
        .save(deps.storage, &Uint128::zero())
        .unwrap();

    // Set up pots with a simulated initial balance
    for pot_id in 1..=5 {
        POT_STATES
            .save(
                deps.storage,
                pot_id,
                &TokenAllocation {
                    pot_id,
                    amount: Uint128::from(1000u128),
                },
            )
            .unwrap();
    }
}
