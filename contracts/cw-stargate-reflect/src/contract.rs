#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, to_vec, Binary, ContractResult, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, SystemResult,
};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg, QueryStargateResponse};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryStargate {
            path,
            query_request,
        } => to_binary(&query_stargate(deps, path, query_request)?),
    }
}

fn query_stargate(
    deps: Deps,
    path: String,
    query_request: String,
) -> StdResult<QueryStargateResponse> {
    let data = Binary::from_base64(&query_request)?;
    let request = &cosmwasm_std::QueryRequest::<cosmwasm_std::Empty>::Stargate { path, data };
    let raw = to_vec(request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
    let value = match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }?
    .to_string();

    Ok(QueryStargateResponse { value })
}
