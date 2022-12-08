use candid::{CandidType, Principal};
use ic_cdk_macros::{self, query, update};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use ic_cdk::api::call::RejectionCode;

pub const DATA_POINTS_PER_API: u64 = 200;

pub const MAX_RESPONSE_BYTES: u64 = 10 * 6 * DATA_POINTS_PER_API;

// todo:
// - send request function
// - transform function

async fn send_request(host: String, url: String, method: HttpMethod, body: Option<Vec<u8>>) -> Result<String, (RejectionCode, String)> {
    let mut host_header = host.clone().to_owned();
    host_header.push_str(":443");

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host_header,
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "oracle_canister".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        method,
        body,
        max_response_bytes: Some(MAX_RESPONSE_BYTES),
        transform: Some(TransformContext::new(transform, vec![])),
        headers: request_headers,
    };

    ic_cdk::api::print(format!("Requesting url: {}", url.to_string()));

    match http_request(request).await {
        Ok((response, )) => {
            ic_cdk::api::print(format!("Response status: {}", response.status));

            let decoded_body = String::from_utf8(response.body)
                .expect("Remote service response is not UTF-8 encoded.");

            ic_cdk::api::print(format!("Response body: {}", decoded_body));

            Ok(decoded_body)
        },
        Err((code, message)) => {
            ic_cdk::api::print(format!("Error: {}", message));
            Err((code, message))
        }
    }
}

#[update]
async fn fetch_price() -> Result<String, String> {
    let host = "api.pro.coinbase.com".to_string();

    let url = format!("https://{host}/products/ICP-USD/stats");
    ic_cdk::api::print(url.clone());

    match send_request(host, url, HttpMethod::GET, None).await {
        Ok(response) => {
            ic_cdk::api::print(format!("Response from fetch_price: {}", response));

            let response_obj: Value = serde_json::from_str(&response).unwrap();

            ic_cdk::api::print(format!("Price: {}", response_obj["last"]));

            Ok(response_obj["last"].to_string())
        }
        Err((code, message)) => {
            let f_message =
                format!("The http_request resulted into error. RejectionCode: {code:?}, Error: {message}");
            ic_cdk::api::print(f_message.clone());

            Err(message)
        }
    }
}

#[update]
async fn send_transaction(_: String, body: Option<Vec<u8>>) -> Result<String, String> {
    let host = "eth-goerli.g.alchemy.com".to_string();

    let url = format!("https://{host}/v2/bUH5A9MJ6basJ88Hq85y23Ada8CYSvD4");
    ic_cdk::api::print(url.clone());

    match send_request(host, url, HttpMethod::POST, body).await {
        Ok(result) => {
            ic_cdk::api::print(result.clone());
            ic_cdk::api::print("Got response from remote service.");

            // let response: Value = serde_json::from_str(&decoded_body).unwrap();

            // ic_cdk::api::print(format!("{}", response["last"]));

            // response["last"].to_string()
            Ok("Ok".to_string())
        }
        Err((code, message)) => {
            let f_message =
                format!("The http_request resulted into error. RejectionCode: {code:?}, Error: {message}");
            ic_cdk::api::print(f_message.clone());

            Err(message)
        }
    }
}

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let mut sanitized = raw.response.clone();
    sanitized.headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];
    sanitized
}

fn main() {}
