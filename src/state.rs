use cosmwasm_std::{Response, Addr, Env};
use cw_storage_plus::Item;
use crate::{data::{DealMetadata}, error::ContractError, execute::Context, msg::InstantiateMsg};
use super::data::Config;

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEAL_METADATA: Item<DealMetadata> = Item::new("deal_metadata");
pub const CREATED_AT: Item<Env> = Item::new("created_at");
pub const CREATED_BY: Item<Addr> = Item::new("created_by");

/// Top-level initialization of contract state
pub fn init_contract(
    ctx: Context,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let metadata = DealMetadata {
        name: msg.name,
        description: msg.description,
        image_url: None, // Placeholder
    };

    DEAL_METADATA.save(ctx.deps.storage, &metadata)?;
    CREATED_AT.save(ctx.deps.storage, &ctx.env.block.time)?;
    CREATED_BY.save(ctx.deps.storage, &ctx.info.sender)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

