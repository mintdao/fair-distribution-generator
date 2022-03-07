use cosmwasm_std::{Addr, DepsMut};

use crate::error::ContractError;
use crate::error::ContractError::Unauthorized;
use crate::state::STATE;

pub fn assert_owner(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let owner = STATE.load(deps.storage).unwrap().owner;
    if owner != sender {
        return Err(Unauthorized {});
    }
    return Ok(());
}
