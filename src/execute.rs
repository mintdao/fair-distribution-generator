use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, WasmMsg};

use crate::error::ContractError;
use crate::msg::FairDistributionInstantiateMsg;
use crate::state::STATE;

pub fn try_update_config(
    deps: DepsMut,
    _info: MessageInfo,
    owner: Option<String>,
    fair_distribution_code_id: Option<u64>,
) -> Result<Response, ContractError> {
    let mut current_state = STATE.load(deps.storage)?;
    if owner.is_some() {
        current_state.owner = deps.api.addr_validate(&owner.unwrap())?;
    }
    if fair_distribution_code_id.is_some() {
        current_state.fair_distribution_code_id = fair_distribution_code_id.unwrap();
    }

    STATE.save(deps.storage, &current_state)?;

    Ok(Response::new().add_attribute("method", "try_update_config"))
}

pub fn try_create_fair_distribution_offering(
    deps: DepsMut,
    env: Env,
    msg: FairDistributionInstantiateMsg,
) -> Result<Response, ContractError> {
    let current_state = STATE.load(deps.storage)?;

    Ok(Response::new()
        .add_attribute("method", "try_create_fair_distribution_offering")
        .add_message(CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id: current_state.fair_distribution_code_id,
            funds: vec![],
            admin: Some(env.contract.address.into_string()),
            label: "Fair Distribution".to_string(),
            msg: to_binary(&msg)?,
        })))
}
