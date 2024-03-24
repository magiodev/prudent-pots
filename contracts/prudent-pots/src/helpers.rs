use cosmwasm_std::{Addr, Env, QuerierWrapper};

use crate::ContractError;

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
