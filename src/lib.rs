use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {
    pub step: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    CurrentValue { address: String },
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}
