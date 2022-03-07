use cosmwasm_std::from_binary;
use cosmwasm_std::testing::{mock_env, mock_info};

use crate::contract::{execute, migrate, query};
use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::tests::utils::{
    assert_error, instantiate_contract, query_contract, InstantiateContractParams, RANDOM_ADDRESS_1,
};

#[test]
fn proper_initialization() {
    let (deps, params, res) = instantiate_contract(InstantiateContractParams {
        msg: InstantiateMsg {
            fair_distribution_code_id: 17,
        },
        ..InstantiateContractParams::default()
    });
    assert_eq!(0, res.messages.len());

    let value = query_contract::<ConfigResponse>(deps.as_ref(), QueryMsg::GetConfig {});
    assert_eq!(
        value.fair_distribution_code_id,
        params.msg.fair_distribution_code_id
    );
    assert_eq!(value.owner, params.contract_creator);
}

#[test]
fn migrate_contract() {
    let (mut deps, _params, _res) = instantiate_contract(InstantiateContractParams {
        ..InstantiateContractParams::default()
    });

    migrate(deps.as_mut(), mock_env(), MigrateMsg {}).unwrap();
}

#[test]
fn update_config() {
    let (mut deps, params, _res) = instantiate_contract(InstantiateContractParams {
        ..InstantiateContractParams::default()
    });
    let info = mock_info(RANDOM_ADDRESS_1, &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        fair_distribution_code_id: Some(15),
    };
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    assert_error(&res, ContractError::Unauthorized {});

    let info = mock_info(&params.contract_creator.to_string(), &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        fair_distribution_code_id: Some(19),
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let value: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(value.fair_distribution_code_id, 19);
    assert_eq!(value.owner, params.contract_creator.to_string());

    let info = mock_info(&params.contract_creator.to_string(), &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(RANDOM_ADDRESS_1.to_string()),
        fair_distribution_code_id: None,
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let value: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(value.owner, RANDOM_ADDRESS_1);
}
