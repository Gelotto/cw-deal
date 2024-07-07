use cosmwasm_schema::cw_serde;

use crate::data::Config;

#[cw_serde]
pub struct InstantiateMsg {
    name: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig(Config),
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);
