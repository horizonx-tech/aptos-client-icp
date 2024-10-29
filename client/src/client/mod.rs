use std::str::FromStr;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::http::{Request, Response};

#[derive(Clone)]
pub struct CallOptions {
    pub max_response_bytes: Option<u64>,
    pub transform: Option<TransformContext>,
}

impl Default for CallOptions {
    fn default() -> Self {
        Self {
            max_response_bytes: None,
            transform: None,
        }
    }
}

// Calcurate cycles for http_request
// NOTE:
//   v0.11: https://github.com/dfinity/cdk-rs/blob/0b14facb80e161de79264c8f88b1a0c8e18ffcb6/examples/management_canister/src/caller/lib.rs#L7-L19
//   v0.8: https://github.com/dfinity/cdk-rs/blob/a8454cb37420c200c7b224befd6f68326a01442e/src/ic-cdk/src/api/management_canister/http_request.rs#L290-L299
fn http_request_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    // The fee is for a 13-node subnet to demonstrate a typical usage.
    (3_000_000u128
        + 60_000u128 * 13
        + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
        + max_response_bytes * 800)
        * 13
}

#[derive(Clone)]
pub struct HttpProvider {}

impl HttpProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl HttpProvider {
    pub async fn send(&self, request: Request, opts: CallOptions) -> Response {
        let body = request.body().unwrap_or_default();
        let method = match request.method() {
            crate::http::HttpMethod::GET => HttpMethod::GET,
            crate::http::HttpMethod::POST => HttpMethod::POST,
            crate::http::HttpMethod::HEAD => HttpMethod::HEAD,
            _ => HttpMethod::GET,
        };
        let mut url = request.url.clone();
        if !request.query().is_empty() {
            let mut queries = request
                .query()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Vec<(String, String)>>();
            let first = queries.remove(0);
            url = format!("{}?{}={}", url, first.0, first.1);
            queries.iter().for_each(|(k, v)| {
                url = format!("&{}?{}={}", url, k, v);
            });
        }

        let headers = request
            .headers()
            .iter()
            .map(|(k, v)| HttpHeader {
                name: k.as_str().to_string(),
                value: v.to_string(),
            })
            .collect();

        let args: CanisterHttpRequestArgument = CanisterHttpRequestArgument {
            body: Some(body.to_vec()),
            url: url,
            headers,
            max_response_bytes: opts.max_response_bytes,
            method,
            transform: opts.transform, //TODO
        };
        let cycles = http_request_required_cycles(&args);
        match http_request(args, cycles).await {
            Ok(response) => {
                let status: u16 = TryFrom::try_from(response.0.status.0).unwrap();
                Response::new(
                    status,
                    String::from_utf8_lossy(response.0.body.as_slice()).to_string(),
                )
            }
            Err(e) => Response::new(500, e.1),
        }
    }
}

#[derive(Clone)]
pub enum Provider {
    Http(HttpProvider),
}

impl Provider {
    pub fn new() -> Self {
        Self::Http(HttpProvider::new())
    }
}
