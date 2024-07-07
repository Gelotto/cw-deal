use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128, Uint64};

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
pub struct ConsiderationMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_urls: Option<Vec<String>>,
}

#[cw_serde]
pub enum ConsiderationItemPartyStatus {
    Pending,
    Accepted,
    Rejected,
}

/// Marketing info and other metadata pertaining to an object of consideration
/// (i.e. something being offered in exchange)
#[cw_serde]
pub struct ConsiderationParty {
    pub address: Addr,
    pub status: ConsiderationItemPartyStatus,
}

/// Item being exchanged.
#[cw_serde]
pub struct Consideration {
    pub asset_ids: Vec<String>,
    /// Marketing & display info
    pub metadata: ConsiderationMetadata,
    /// The initiating party & counterparty
    pub parties: Vec<ConsiderationParty>,
    /// Structured config for how the item shall be exchanged
    pub params: ExchangeParams,
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
pub struct DealMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[cw_serde]
pub struct Deal {
    pub id: String,
    /// Time the deal was initialized
    pub created_at: Timestamp,
    /// Address of person to initialize the deal
    pub created_by: Addr,
    /// Marketing info & other metadata
    pub metadata: DealMetadata,
    /// List of all parties included in the deal
    pub parties: Vec<Party>,
    /// List of all items being exchanged in the deal
    pub items: Vec<Consideration>,
}

/// Item being exchanged.
#[cw_serde]
pub enum Asset {
    TokenAmount {
        denom: String,
        amount: Uint128,
    },
    Nft {
        collection_addr: Addr,
        token_id: String,
    },
    Thing {
        id: String,
        name: String,
    },
}
