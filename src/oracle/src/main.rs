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


    let call_result: Result<(Result<Vec<u8>, String>,), _> =
        ic_cdk::api::call::call(
            Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "change_price",
            (11155111 as u64, 10 as u64, 21000 as u64, 300 as u64, "0xCFf00E5f685cCE94Dfc6d1a18200c764f9BCca1f", price)
        ).await;

    let tx = call_result.unwrap().0.unwrap();

    let decoded_tx = String::from_utf8(tx)
        .expect("Remote service response is not UTF-8 encoded.");

    ic_cdk::api::print(format!("Signed tx: {}", decoded_tx));
}

fn main() {}
