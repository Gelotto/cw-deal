use crate::{error::ContractError, msg::ConfigResponse, state::{CONFIG, DEAL}};
use super::ReadonlyContext;

pub fn query_config(ctx: ReadonlyContext) -> Result<ConfigResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(CONFIG
        .load(deps.storage)
        .and_then(|config| Ok(ConfigResponse(config)))?)
}

pub fn query_deal(ctx: ReadonlyContext) -> Result<Deal, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    DEAL.load(deps.storage).map_err(|err| ContractError::Std(err))
}

