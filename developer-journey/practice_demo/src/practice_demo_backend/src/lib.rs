mod types;

use ic_ledger_types::Tokens;
use types::*;

#[allow(dead_code)]
#[ic_cdk::query]
fn config() -> ConfigResponse {
    let config_response = ConfigResponse {
        purchase_price: Tokens::from_e8s(10),
        transfer_price: Tokens::from_e8s(20),
    };
    config_response
}

#[ic_cdk::query]
fn resolve_record(arg: String) -> ResolveRecordResponse {
    let resolve_record_response = ResolveRecordResponse {
        address: Some(String::from(arg+" abc")),
    };
    resolve_record_response
}

#[ic_cdk::update]
fn register(domain_name : String) -> RegisterReceipt {
    let resolve_record_response = ResolveRecordResponse {
        address: Some(String::from(domain_name+" abc")),
    };
    RegisterReceipt::Ok(resolve_record_response)
}

#[ic_cdk::update]
fn transfer(domain_name: String, new_name : String) -> TransferReceipt {
    let s = Box::leak(new_name.into_boxed_str());
    let s_slice: &str = &s[..]; 
    let output = domain_name + " " + s_slice + " abc";
    let resolve_record_response = ResolveRecordResponse {
        address: Some(String::from(output)),
    };
    TransferReceipt::Ok(resolve_record_response)
}
