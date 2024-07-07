use crate::{error::ContractError, msg::ConfigResponse, state::{CONFIG, DEAL_METADATA, CREATED_AT, CREATED_BY}};
use cosmwasm_std::Env;
use super::ReadonlyContext;

pub fn query_config(ctx: ReadonlyContext) -> Result<ConfigResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(CONFIG
        .load(deps.storage)
        .and_then(|config| Ok(ConfigResponse(config)))?)
}

pub fn query_deal(ctx: ReadonlyContext) -> Result<Deal, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    
    let metadata = DEAL_METADATA.load(deps.storage)?;
    let created_at = CREATED_AT.load(deps.storage)?;
    let created_by = CREATED_BY.load(deps.storage)?;
    
    let deal = Deal {
        id: "dynamic_deal".to_string(), // This can be adjusted as needed
        created_at,
        created_by,
        metadata,
        parties: vec![], // Placeholder: Populate as per your logic
        items: vec![],   // Placeholder: Populate as per your logic
    };
    
    Ok(deal)
}

