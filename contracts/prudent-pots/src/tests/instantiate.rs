#[cfg(test)]
pub mod tests {
    use crate::{
        contract::instantiate,
        msg::InstantiateMsg,
        state::{GameConfig, TokenAllocation, PLAYER_ALLOCATIONS, POT_STATES},
    };
    use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, StdError, Storage, Uint128};

    // Fixture methods

    pub fn setup_game_no_raffle_works(
        mut deps: DepsMut,
        env: &Env,
        info: MessageInfo,
        pot_allocations: Option<Vec<(u8, Addr, Uint128)>>,
    ) {
        // Define the game configuration
        let config = GameConfig {
            game_duration: 3600,
            game_extend: 600,
            fee: 2,
            fee_reallocation: 5,
            fee_address: Addr::unchecked("fee_address"),
            game_denom: "token".to_string(),
            game_cw721_addrs: vec![Addr::unchecked("nft")],
            min_pot_initial_allocation: Uint128::new(200u128),
            decay_factor: Uint128::new(95u128),
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
        let mut player_allocations = PLAYER_ALLOCATIONS
            .load(storage, player.to_string())
            .unwrap_or(vec![]);

        player_allocations.push(TokenAllocation { pot_id, amount });
        PLAYER_ALLOCATIONS
            .save(storage, player.to_string(), &player_allocations)
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

// TODO: setup_game_raffle_works
