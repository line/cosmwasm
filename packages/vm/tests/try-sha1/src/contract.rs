use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::msg::InstantiateMsg;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let _ = deps.api.sha1_calculate(&[&[]])?;
    Ok(Response::default())
}
