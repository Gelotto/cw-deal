use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp, Uint64};

#[cw_serde]
pub struct Config {}

/// Manner in which an item is exchanged btw parties
#[cw_serde]
pub enum ExchangeParams {
    Sale {
        /// Payment token denom and amount
        price: Coin,
    },
    Loan {
        /// Loan time in seconds
        duration: Option<Uint64>,
    },
    Donation {},
    Trade {},
}

/// Marketing info and other metadata pertaining to an object of consideration
/// (i.e. something being offered in exchange)
#[cw_serde]
pub struct ConsiderationItemMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_urls: Option<Vec<String>>,
}

/// Item being exchanged.
#[cw_serde]
pub struct ConsiderationItem {
    pub id: String,
    /// Marketing & display info
    pub metadata: ConsiderationItemMetadata,
    /// How the item is exchanged
    pub params: ExchangeParams,
    /// The offering party
    pub owner: Addr,
    /// The receiving party
    pub recipient: Addr,
}

#[cw_serde]
pub struct PartyMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[cw_serde]
pub struct Party {
    pub address: Addr,
    pub metadata: PartyMetadata,
}

#[cw_serde]
pub struct Deal {
    pub id: String,
    /// Time the deal was initialized
    pub created_at: Timestamp,
    /// Address of person to initialize the deal
    pub created_by: Addr,
    /// List of all parties included in the deal
    pub parties: Vec<Party>,
    /// List of all items being exchanged in the deal
    pub items: Vec<ConsiderationItem>,
}
