use cosmwasm_std::Response;
use cw_storage_plus::Item;

use crate::{error::ContractError, execute::Context, msg::InstantiateMsg};

use super::data::Config;

pub const CONFIG: Item<Config> = Item::new("config");

/// Top-level initialization of contract state
pub fn init_contract(
    _ctx: Context,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}
