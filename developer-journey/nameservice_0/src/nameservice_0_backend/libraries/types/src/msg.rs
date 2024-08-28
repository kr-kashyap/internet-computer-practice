use candid::CandidType;
use serde::{Deserialize, Serialize};

use ic_ledger_types::{Tokens};

// use cosmwasm_schema::{QueryResponses}; // TODO

#[derive(CandidType, Serialize, Deserialize)]
pub struct InstantiateMsg {
    pub purchase_price: Option<Tokens>,
    pub transfer_price: Option<Tokens>,
    // pub cw20_contract: String, // Check
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum ExecuteMsg {
    Register {
        name: String,
        coin: Tokens,
    },
    Transfer {
        name: String,
        to: String,
        coin: Tokens,
    },
}

#[derive(CandidType, Serialize, Deserialize)] //, QueryResponses)] // TODO
pub enum QueryMsg {
    // ResolveAddress returns the current address that the name resolves to
    #[returns(ResolveRecordResponse)]
    ResolveRecord { name: String },
    #[returns(ConfigResponse)]
    Config {},
}

// We define a custom struct for each query response
#[derive(CandidType, Serialize, Deserialize)]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub purchase_price: Option<Tokens>,
    pub transfer_price: Option<Tokens>,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> ConfigResponse {
        ConfigResponse {
            purchase_price: config.purchase_price,
            transfer_price: config.transfer_price,
        }
    }
}
// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();