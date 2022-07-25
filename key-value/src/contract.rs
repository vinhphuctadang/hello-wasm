use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

// namespace msg in this cope
// crate: represent this crate
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetNameResponse};
use crate::state::{resolver, resolver_read, Value};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetNameMsg { key, value } => execute_set(deps, env, info, key, value),
    }
}

fn execute_set(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    key: String, value: String,
) -> Result<Response, ContractError> {
    let key_bytes = key.as_bytes();
    let store_state = Value { value: value };

    resolver(deps.storage).save(key_bytes, &store_state)?; // ignore error
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    query_msg: QueryMsg
) -> StdResult<Binary> {
    match query_msg {
        QueryMsg::GetName { name } => query_resolver(deps, env, name)
    }
}

pub fn query_resolver(deps: Deps, _env: Env, name: String) -> StdResult<Binary> {
    let key = name.as_bytes();
    let name = match resolver_read(deps.storage).may_load(key)? {
        Some(value) => Some(String::from(&value.value)),
        None => None
    };
    let resp = GetNameResponse { name };
    to_binary(&resp)
}
