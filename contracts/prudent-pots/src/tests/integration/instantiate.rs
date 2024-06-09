// use cosmwasm_std::{coin, Uint128};

// use crate::{
//     state::Raffle,
//     tests::integration::fixtures::{default_with_balances, DENOM_GAME},
// };

// TODO_FUTURE: Those cases are not very consistent now that we are not isntantiating with raffle.

// #[test]
// fn test_instantiate_simple_works() {
//     let (mut app, pp_addr, cw721_addr) =
//         default_with_balances(1, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

//     todo!()
// }

// #[test]
// fn test_instantiate_raffle_cw721_works() {
//     let (mut app, pp_addr, cw721_addr) = default_with_balances(
//         1,
//         vec![coin(100_000_000u128, DENOM_GAME.to_string())],
//         Some(Raffle {
//             cw721_token_ids: vec![1],
//             denom_amount: Uint128::new(0u128),
//         }),
//     );

//     todo!()
// }

// #[test]
// fn test_instantiate_raffle_cw721_multiple_works() {
//     let (mut app, pp_addr, cw721_addr) = default_with_balances(
//         1,
//         vec![coin(100_000_000u128, DENOM_GAME.to_string())],
//         Some(Raffle {
//             cw721_token_ids: vec![1, 2, 3, 4, 5],
//             denom_amount: Uint128::new(0u128),
//         }),
//     );

//     todo!()
// }

// #[test]
// fn test_instantiate_raffle_denom_works() {
//     let (mut app, pp_addr, cw721_addr) = default_with_balances(
//         1,
//         vec![coin(100_000_000u128, DENOM_GAME.to_string())],
//         Some(Raffle {
//             cw721_token_ids: vec![],
//             denom_amount: Uint128::new(100_000_000u128),
//         }),
//     );
//     todo!()
// }

// #[test]
// fn test_instantiate_raffle_both_works() {
//     let (mut app, pp_addr, cw721_addr) = default_with_balances(
//         1,
//         vec![coin(100_000_000u128, DENOM_GAME.to_string())],
//         Some(Raffle {
//             cw721_token_ids: vec![1],
//             denom_amount: Uint128::new(100_000_000u128),
//         }),
//     );
//     todo!()
// }

// #[test]
// fn test_instantiate_raffle_both_multiple_works() {
//     let (mut app, pp_addr, cw721_addr) = default_with_balances(
//         1,
//         vec![coin(100_000_000u128, DENOM_GAME.to_string())],
//         Some(Raffle {
//             cw721_token_ids: vec![1, 2, 3, 4, 5],
//             denom_amount: Uint128::new(100_000_000u128),
//         }),
//     );

//     todo!()
// }
