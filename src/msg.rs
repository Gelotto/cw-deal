use cosmwasm_schema::cw_serde;

use crate::data::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig(Config),
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Deal {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

