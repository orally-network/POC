use ic_cdk::api::call::CallResult;
use ic_cdk_macros::{self, query, update};
use ic_cdk::export::{Principal};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};

#[update]
async fn make_magic() -> () {
    let call_result: Result<(Result<String, String>,), _> =
        ic_cdk::api::call::call(
            Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            "fetch_price",
            ()
        ).await;

    let price = call_result.unwrap().0.unwrap();

    ic_cdk::api::print(format!("Price from oracle: {}", price));
}

fn main() {}
