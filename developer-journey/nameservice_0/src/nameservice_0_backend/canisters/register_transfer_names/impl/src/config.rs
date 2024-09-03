use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::query;
use register_transfer_names_api::config::{Response::*, *};

#[query]
fn config() -> Response {
    RUNTIME_STATE.with(|state| config_impl(&state.borrow()))
}

fn config_impl(runtime_state: &RuntimeState) {
    println!("Inside");
    // let caller = runtime_state.env.caller();
    // if runtime_state.data.push_service_principals.contains(&caller) {
    //     let config = runtime_state
    //         .data
    //         .config
    
    //     Success(SuccessResult { config })
    // } else {
    //     NotAuthorized
    // }
}
