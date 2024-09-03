use candid::CandidType;
use ic_ledger_types::Tokens;

#[allow(dead_code)]
#[derive(CandidType)]
pub struct ConfigResponse {
    pub purchase_price: Tokens,
    pub transfer_price: Tokens,
}

// We define a custom struct for each query response
#[derive(CandidType)]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}

#[derive(CandidType)]
pub struct Register {
    name: String,
    coin: Tokens,
}

#[derive(CandidType)]
pub struct Transfer {
    name: String,
    to: String,
    coin: Tokens,
}

pub type RegisterReceipt = Result<ResolveRecordResponse, RegisterErr>;

#[derive(CandidType)]
pub enum RegisterErr {
    NotAllowed,
    InsufficientTokens,
}

pub type TransferReceipt = Result<ResolveRecordResponse, TransferErr>;

#[derive(CandidType)]
pub enum TransferErr {
    NotAllowed,
    NotExistingDomain,
    InsufficientTokens,
}

// TODO
// #[derive(CandidType)]
// pub struct InstantiateMsg {
//     pub purchase_price: Option<Tokens>,
//     pub transfer_price: Option<Tokens>,
//     // pub cw20_contract: String, // Check
// }

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();