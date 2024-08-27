use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize)]
pub struct InstantiateMsg {
    pub purchase_price: Uint,
    // : Option<Coin>,  // TODO
    pub transfer_price: Uint
    // : Option<Coin>,  // TODO
    // pub cw20_contract: String, // Check
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum ExecuteMsg {
    Register {
        name: String,
        coin: Uint,
        // : Coin, // TODO
    },
    Transfer {
        name: String,
        to: String,
        coin: Uint,
        // : Coin, // TODO
    },
}

#[derive(CandidType, Serialize, Deserialize, QueryResponses)]
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
    pub purchase_price: Uint,
    // : Option<Coin>,  // TODO
    pub transfer_price: Uint,
    // : Option<Coin>,  // TODO
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> ConfigResponse {
        ConfigResponse {
            purchase_price: config.purchase_price,
            transfer_price: config.transfer_price,
        }
    }
}
