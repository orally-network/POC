use ic_cdk::api::call::CallResult;
use ic_cdk_macros::{self, query, update};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};

#[update]
async fn make_magic() -> () {
    // let result = ic_cdk::api::call::call_raw(
    //     "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string().parse().unwrap(),
    //     "fetch_price",
    //     &[][..],
    //     2_000_000_000,
    // ).await;

    let result = ic_cdk::api::call::call("ryjl3-tyaaa-aaaaa-aaaba-cai".parse().unwrap(), "fetch_price", ()).await;

    let price: String = candid::utils::decode_one(&result.unwrap()).expect("fetch_price failed!");

    ic_cdk::api::print(format!("Price from oracle: {}", price));
}

fn main() {}
