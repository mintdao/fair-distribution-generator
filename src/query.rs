use cosmwasm_std::{Deps, StdResult};

use crate::msg::ConfigResponse;
use crate::state::STATE;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let current_state = STATE.load(deps.storage)?;

    Ok(ConfigResponse {
        owner: current_state.owner,
        fair_distribution_code_id: current_state.fair_distribution_code_id,
    })
}
