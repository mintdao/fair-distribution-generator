use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{from_binary, Deps, OwnedDeps, Response};
use serde::de::DeserializeOwned;

use crate::contract::{instantiate, query};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg};

#[derive(Debug)]
pub struct InstantiateContractParams {
    pub msg: InstantiateMsg,
    pub contract_creator: String,
}

pub const RANDOM_ADDRESS_1: &str = "terra1ykp6wnp2tlz3r2nynd5yph60zu08l83vhgplee";

impl InstantiateMsg {
    pub fn default() -> Self {
        Self {
            fair_distribution_code_id: 0,
        }
    }
}

impl InstantiateContractParams {
    pub fn default() -> Self {
        Self {
            msg: InstantiateMsg {
                ..InstantiateMsg::default()
            },
            contract_creator: "creator".to_string(),
        }
    }
}

pub fn instantiate_contract(
    params: InstantiateContractParams,
) -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    InstantiateContractParams,
    Response,
) {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(params.contract_creator.as_str(), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, params.msg.clone()).unwrap();
    return (deps, params, res);
}

pub fn query_contract<T: DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    let res = query(deps, mock_env(), msg).unwrap();
    return from_binary::<T>(&res).unwrap();
}

pub fn assert_error(res: &Result<Response, ContractError>, contract_err: ContractError) {
    match res {
        Err(thrown_error) => {
            assert_eq!(thrown_error, &contract_err);
        }
        _ => {
            panic!("Should throw {:?} error", contract_err)
        }
    }
}
