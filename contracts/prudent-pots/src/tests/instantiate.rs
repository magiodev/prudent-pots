#[cfg(test)]
pub mod tests {
    use crate::{
        contract::instantiate,
        msg::InstantiateMsg,
        state::{GameConfig, PlayerAllocations, TokenAllocation, PLAYER_ALLOCATIONS, POT_STATES},
    };
    use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, StdError, Storage, Uint128};

    // Fixture methods

    pub fn setup_game(
        mut deps: DepsMut,
        env: &Env,
        info: MessageInfo,
        pot_allocations: Option<Vec<(u8, Addr, Uint128)>>,
    ) {
        // Define the game configuration
        let config = GameConfig {
            game_duration: 3600,
            game_extend: 600,
            fee_allocation: 2,
            fee_reallocation: 5,
            fee_allocation_address: Addr::unchecked("fee_address"),
            game_denom: "token".to_string(),
            min_bid: Uint128::new(1000000u128),
        };

        // Perform instantiation first
        let _ = instantiate(
            deps.branch(),
            env.clone(),
            info.clone(),
            InstantiateMsg {
                config: config.clone(),
            },
        )
        .unwrap();

        // Since instantiate consumes deps, we need to extract storage again from deps after instantiation
        let storage = deps.storage;

        // Set up pot allocations after instantiation, so they don't get cleared
        if let Some(allocations) = pot_allocations {
            for (pot_id, player, amount) in allocations {
                setup_pot_allocation(storage, pot_id, &player, amount);
            }
        }
    }

    fn setup_pot_allocation(storage: &mut dyn Storage, pot_id: u8, player: &Addr, amount: Uint128) {
        let mut player_allocations =
            PLAYER_ALLOCATIONS
                .load(storage, player.clone())
                .unwrap_or(PlayerAllocations {
                    allocations: vec![],
                });

        player_allocations
            .allocations
            .push(TokenAllocation { pot_id, amount });
        PLAYER_ALLOCATIONS
            .save(storage, player.clone(), &player_allocations)
            .unwrap();

        // Update the pot state to reflect the new allocation
        POT_STATES
            .update(storage, pot_id, |pot_state| -> Result<_, StdError> {
                let mut state = pot_state.unwrap_or(TokenAllocation {
                    pot_id,
                    amount: Uint128::zero(),
                });
                state.amount += amount;
                Ok(state)
            })
            .unwrap();
    }
}
