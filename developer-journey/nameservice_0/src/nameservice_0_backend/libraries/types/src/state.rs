// use cw_storage_plus::{Item, Map}; // TODO

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize)]
pub struct Config {
    pub purchase_price: Option<Tokens>,
    pub transfer_price: Option<Tokens>,
    // pub cw20_contract: String, // Check
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct NameRecord {
    pub owner: String,
}

// pub const CONFIG // Check
// : Item<Config> = Item::new("config");
// pub const NAME_RESOLVER // Check
// : Map<&[u8], NameRecord> = Map::new("name_resolver");
