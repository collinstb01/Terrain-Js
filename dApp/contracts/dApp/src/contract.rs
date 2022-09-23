#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;
use schemars::_serde_json::Result;

use crate::error::ContractError;
use crate::msg::{OpenentResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, ENTRIES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dApp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        opponent: msg.opponent,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        // .add_attribute("opponent", msg.opponent.to_string())
        .add_attribute("owner", info.sender.clone()))
    // OK(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: Deps,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::EnterRaffle { entry_address } => try_enter_raffle(deps, info, entry_address),
    }
}

pub fn try_enter_raffle(deps: Deps, info: MessageInfo, entry_address: String) {
    // 
    // check if user is the owner
    // if not the owner, check to make sure the entry_address = info.sender
    // verify address and the user address 
    // 

    let address = deps.api.add_attribute(entry_address)?;

    let increment_entry = |nums_data: Option<u8> | -> StdResult<Response, ContractError> {
        nums_data + 1;
    };

let state = STATE.load(deps.storage)?;
    if info.sender == address {
        return Err(ContractError::Unauthorized {});
          if  info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
       } else {
        match ENTRIES.may_load(deps.storage, &address)? {
            None =>ENTRIES.save(&mut deps.storage, &address, 1) ,
            Some(value) => ENTRIES.update(&mut deps.storage, &address, increment_entry)?
        }
            // add address to entries
       }
    } else {
    //    if  info.sender != state.owner {
    //         return Err(ContractError::Unauthorized {});
    //    } else {
    //     match ENTRIES.may_load(deps.storage, &address)? {
    //         None =>ENTRIES.save(deps.storage, &address, 1) ,
    //         Some(value) => ENTRIES.update(deps.storage, &address, increment_entry)
    //     }
    //         // add address to entries
    //    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg,) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetOpenent {} => to_binary(&query_opponent(deps)?),
//     }
// }

pub fn query_opponent(deps: Deps) -> StdResult<Binary> {
    // STATE.load(deps, )
    Ok(Response::new().add_attribute("method", "Queried Successfully"))
}
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        // let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        // let info = mock_info("creator", &coins(1000, "earth"));
        // let msg = InstantiateMsg { opponent: "address".to_string() };
        // // we can just call .unwrap() to assert this was a success
        // let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn check_if_valid_addr() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let info = mock_info("creator", &coins(2, "token"));
        let msg = InstantiateMsg { opponent: "info".to_string() };
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // let rec: Addr = deps.api.addr_validate(msg.opponent);

    }
}
