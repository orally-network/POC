use candid::{CandidType, Principal};
use ic_cdk_macros::{self, query, update};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use ic_cdk::api::call::RejectionCode;

#[derive(CandidType, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, CandidType, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    HEAD,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CanisterHttpRequestArgs {
    pub url: String,
    pub max_response_bytes: Option<u64>,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
    pub http_method: HttpMethod,
    pub transform_method_name: Option<String>,
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanisterHttpResponsePayload {
    pub status: u64,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

pub const DATA_POINTS_PER_API: u64 = 200;

pub const MAX_RESPONSE_BYTES: u64 = 10 * 6 * DATA_POINTS_PER_API;

// todo:
// - send request function
// - transform function

#[query]
async fn transform(raw: CanisterHttpResponsePayload) -> CanisterHttpResponsePayload {
    let mut sanitized = raw.clone();
    sanitized.headers = vec![];
    sanitized
}

async fn send_request(url: String, http_method: HttpMethod, headers: Vec<HttpHeader>, body: Option<Vec<u8>>) -> Result<Vec<u8>, (RejectionCode, String)> {
    let request = CanisterHttpRequestArgs {
        url: url.clone(),
        http_method,
        body,
        max_response_bytes: Some(MAX_RESPONSE_BYTES),
        transform_method_name: Some("transform".to_string()),
        headers,
    };

    let body = candid::utils::encode_one(&request).unwrap();

    ic_cdk::api::print(format!("Requesting url: {}", url.to_string()));

    ic_cdk::api::call::call_raw(
        Principal::management_canister(),
        "http_request",
        &body[..],
        2_000_000_000,
    ).await
}

#[update]
async fn fetch_price() -> String {
    let host = "api.pro.coinbase.com";
    let mut host_header = host.clone().to_owned();
    host_header.push_str(":443");

    let url = format!("https://{host}/products/ICP-USD/stats");
    ic_cdk::api::print(url.clone());

    // prepare system http_request call
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host_header,
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];

    match send_request(url, HttpMethod::GET, request_headers, None).await {
        Ok(result) => {
            let decoded_result: CanisterHttpResponsePayload =
                candid::utils::decode_one(&result).expect("IC http_request failed!");

            let decoded_body = String::from_utf8(decoded_result.body)
                .expect("Remote service response is not UTF-8 encoded.");

            ic_cdk::api::print(decoded_body.clone());
            ic_cdk::api::print("Got response from remote service.");

            let response: Value = serde_json::from_str(&decoded_body).unwrap();

            ic_cdk::api::print(format!("{}", response["last"]));

            response["last"].to_string()
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            ic_cdk::api::print(message.clone());

            "0".to_string()
        }
    }
}

#[update]
async fn send_transaction(body: Option<Vec<u8>>) -> () {
    let host = "alchemy...";
    let mut host_header = host.clone().to_owned();
    host_header.push_str(":443");

    let url = format!("https://{host}/yoyo...");
    ic_cdk::api::print(url.clone());

    // prepare system http_request call
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host_header,
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];

    match send_request(url, HttpMethod::POST, request_headers, body).await {
        Ok(result) => {
            let decoded_result: CanisterHttpResponsePayload =
                candid::utils::decode_one(&result).expect("IC http_request failed!");

            let decoded_body = String::from_utf8(decoded_result.body)
                .expect("Remote service response is not UTF-8 encoded.");

            ic_cdk::api::print(decoded_body.clone());
            ic_cdk::api::print("Got response from remote service.");

            // let response: Value = serde_json::from_str(&decoded_body).unwrap();

            // ic_cdk::api::print(format!("{}", response["last"]));

            // response["last"].to_string()
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            ic_cdk::api::print(message.clone());
        }
    }
}

fn main() {}
