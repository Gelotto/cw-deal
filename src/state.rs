use cosmwasm_std::Response;
use cw_storage_plus::Item;

use crate::{data::DealMetadata, error::ContractError, execute::Context, msg::InstantiateMsg};

use super::data::Config;

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEAL_METADATA: Item<DealMetadata> = Item::new("deal_metadata");

/// Top-level initialization of contract state
pub fn init_contract(
    _ctx: Context,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}
