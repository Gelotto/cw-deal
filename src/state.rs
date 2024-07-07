use cosmwasm_std::{Response, Addr, Timestamp};
use cw_storage_plus::Item;
use crate::{data::{DealMetadata, Deal}, error::ContractError, execute::Context, msg::InstantiateMsg};
use super::data::Config;

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEAL_METADATA: Item<DealMetadata> = Item::new("deal_metadata");
pub const DEAL: Item<Deal> = Item::new("deal");

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
    
    let deal = Deal {
        id: "initial_deal".to_string(), // Placeholder
        created_at: Timestamp::from_seconds(ctx.env.block.time.seconds()),
        created_by: ctx.info.sender.clone(),
        metadata,
        parties: vec![],
        items: vec![],
    };
    
    DEAL.save(ctx.deps.storage, &deal)?;
    
    Ok(Response::new().add_attribute("action", "instantiate"))
}

