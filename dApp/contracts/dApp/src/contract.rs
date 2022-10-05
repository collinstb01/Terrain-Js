#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use schemars::_serde_json::value;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use crate::state::{State, ENTRIES, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my_terra_dapp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::EnterRaffle { entry_address } => try_enter_raffle(entry_address, info, deps),
        // ExecuteMsg::StartGame {} => try_start_game(),
    }
}
// pub fn try_start_game() {}

pub fn try_enter_raffle(
    entry_address: String,
    info: MessageInfo,
    deps: DepsMut,
) -> Result<Response, ContractError> {
    let update_storage = |d: Option<i32>| -> Result<i32, ContractError> {
        match d {
            Some(value) => Ok(value + 1),
            None => Ok(1),
        }
    };
    let address = deps.api.addr_validate(&entry_address)?;
    let state = STATE.load(deps.storage)?;

    if info.sender == entry_address {
        if info.sender != state.owner {
        } else {
            ENTRIES.update(deps.storage, &address, update_storage)?;
        }
    } else {
        ENTRIES.update(deps.storage, &address, update_storage)?;
    }
    Ok(Response::new().add_attribute("some", "value"))
}

// pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         state.count += 1;
//         Ok(state)
//     })?;

//     Ok(Response::new().add_attribute("method", "try_increment"))
// }

// pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         if info.sender != state.owner {
//             return Err(ContractError::Unauthorized {});
//         }
//         state.count = count;
//         Ok(state)
//     })?;
//     Ok(Response::new().add_attribute("method", "reset"))
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwners {} => to_binary(&query_owner(deps)?),
    }
}

// Funcs for Query

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse {
        owners: state.owner,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Api};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("creator", &coins(1000, "earth"));

        let msg = InstantiateMsg {};

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    #[test]
    fn try_enter_raffle() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("creator", &coins(1000, "earth"));

        let msg = InstantiateMsg {};

        let address = deps.api.addr_validate("jayden").unwrap();
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let state = ENTRIES.load(&deps.storage, &address).unwrap();

        ExecuteMsg::EnterRaffle {
            entry_address: "Hello".to_string(),
        };

        let state2 = ENTRIES.load(&deps.storage, &address).unwrap();

        assert_eq!(state, state2);

        // let address = deps.api.addr_validate()?;

        // we can just call .unwrap() to assert this was a success
    }
}
