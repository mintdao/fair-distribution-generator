use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub fair_distribution_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        fair_distribution_code_id: Option<u64>,
    },
    CreateFairDistributionOffering(FairDistributionInstantiateMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: Addr,
    pub fair_distribution_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FairDistributionInstantiateMsg {
    pub owner: String,
    pub place_order_fee: Uint128,
    pub max_order_size: Uint128,
    pub max_orders_per_wallet: Uint128,
    pub vesting_duration: Uint128,
    pub liquidity: Liquidity,
    pub oracle_address: String,
    pub ste_gateway_address: String,
    pub min_bond_stt: Uint128,
    pub allowed_allocation_per_wallet_with_bonded_stt: Uint128,
    pub max_max_price: Uint128,
    pub pair_swap_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Liquidity {
    pub ust_amount: Uint128,
    pub token_amount: Uint128,
    pub token_address: String,
}
