use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{to_binary, CosmosMsg, SubMsg, WasmMsg};

use crate::contract::execute;
use crate::msg::{ExecuteMsg, FairDistributionInstantiateMsg, Liquidity};
use crate::tests::utils::{instantiate_contract, InstantiateContractParams, RANDOM_ADDRESS_1};

#[test]
fn should_execute_fair_distribution_instantiate_message() {
    let (mut deps, params, _res) = instantiate_contract(InstantiateContractParams {
        ..InstantiateContractParams::default()
    });

    let info = mock_info(RANDOM_ADDRESS_1, &[]);
    let instantiate_msg = FairDistributionInstantiateMsg {
        owner: RANDOM_ADDRESS_1.to_string(),
        place_order_fee: Default::default(),
        max_order_size: Default::default(),
        max_orders_per_wallet: Default::default(),
        vesting_duration: Default::default(),
        liquidity: Liquidity {
            ust_amount: Default::default(),
            token_amount: Default::default(),
            token_address: "".to_string(),
        },
        oracle_address: "".to_string(),
        ste_gateway_address: "".to_string(),
        min_bond_stt: Default::default(),
        allowed_allocation_per_wallet_with_bonded_stt: Default::default(),
        max_max_price: Default::default(),
        pair_swap_fee: Default::default(),
    };
    let msg = ExecuteMsg::CreateFairDistributionOffering(instantiate_msg.clone());
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: Some(mock_env().contract.address.to_string()),
            code_id: params.msg.fair_distribution_code_id,
            msg: to_binary(&instantiate_msg).unwrap(),
            funds: vec![],
            label: "Fair Distribution".to_string(),
        }))]
    )
}
