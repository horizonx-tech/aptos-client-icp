/*
 * Aptos Node API
 *
 * The Aptos Node API is a RESTful API for client applications to interact with the Aptos blockchain.
 *
 * The version of the OpenAPI document: 1.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`encode_submission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EncodeSubmissionError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`estimate_gas_price`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateGasPriceError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountTransactionsError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status410(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transaction_by_hash`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionByHashError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status410(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transaction_by_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionByVersionError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status410(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionsError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status410(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`simulate_transaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimulateTransactionError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status413(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    Status507(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_batch_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitBatchTransactionsError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status413(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    Status507(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_transaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitTransactionError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status413(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    Status507(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`wait_transaction_by_hash`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaitTransactionByHashError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status410(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// This endpoint accepts an EncodeSubmissionRequest, which internally is a UserTransactionRequestInner (and optionally secondary signers) encoded as JSON, validates the request format, and then returns that request encoded in BCS. The client can then use this to create a transaction signature to be used in a SubmitTransactionRequest, which it then passes to the /transactions POST endpoint.  To be clear, this endpoint makes it possible to submit transaction requests to the API from languages that do not have library support for BCS. If you are using an SDK that has BCS support, such as the official Rust, TypeScript, or Python SDKs, you do not need to use this endpoint.  To sign a message using the response from this endpoint: - Decode the hex encoded string in the response to bytes. - Sign the bytes to create the signature. - Use that as the signature field in something like Ed25519Signature, which you then use to build a TransactionSignature.
pub async fn encode_submission(
    configuration: &configuration::Configuration,
    encode_submission_request: models::EncodeSubmissionRequest,
) -> Result<String, Error<EncodeSubmissionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transactions/encode_submission",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&encode_submission_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp: crate::http::Response = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EncodeSubmissionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gives an estimate of the gas unit price required to get a transaction on chain in a reasonable amount of time. The gas unit price is the amount that each transaction commits to pay for each unit of gas consumed in executing the transaction. The estimate is based on recent history: it gives the minimum gas that would have been required to get into recent blocks, for blocks that were full. (When blocks are not full, the estimate will match the minimum gas unit price.)  The estimation is given in three values: de-prioritized (low), regular, and prioritized (aggressive). Using a more aggressive value increases the likelihood that the transaction will make it into the next block; more aggressive values are computed with a larger history and higher percentile statistics. More details are in AIP-34.
pub async fn estimate_gas_price(
    configuration: &configuration::Configuration,
) -> Result<models::GasEstimation, Error<EstimateGasPriceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/estimate_gas_price", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EstimateGasPriceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves on-chain committed transactions from an account. If the start version is too far in the past, a 410 will be returned.  If no start version is given, it will start at version 0.  To retrieve a pending transaction, use /transactions/by_hash.
pub async fn get_account_transactions(
    configuration: &configuration::Configuration,
    address: &str,
    start: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<models::Transaction>, Error<GetAccountTransactionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/accounts/{address}/transactions",
        local_var_configuration.base_path,
        address = crate::apis::urlencode(address)
    );
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAccountTransactionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Look up a transaction by its hash. This is the same hash that is returned by the API when submitting a transaction (see PendingTransaction).  When given a transaction hash, the server first looks for the transaction in storage (on-chain, committed). If no on-chain transaction is found, it looks the transaction up by hash in the mempool (pending, not yet committed).  To create a transaction hash by yourself, do the following: 1. Hash message bytes: \"RawTransaction\" bytes + BCS bytes of [Transaction](https://aptos-labs.github.io/aptos-core/aptos_types/transaction/enum.Transaction.html). 2. Apply hash algorithm `SHA3-256` to the hash message bytes. 3. Hex-encode the hash bytes with `0x` prefix.
pub async fn get_transaction_by_hash(
    configuration: &configuration::Configuration,
    txn_hash: &str,
) -> Result<models::Transaction, Error<GetTransactionByHashError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transactions/by_hash/{txn_hash}",
        local_var_configuration.base_path,
        txn_hash = crate::apis::urlencode(txn_hash)
    );
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTransactionByHashError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a transaction by a given version. If the version has been pruned, a 410 will be returned.
pub async fn get_transaction_by_version(
    configuration: &configuration::Configuration,
    txn_version: &str,
) -> Result<models::Transaction, Error<GetTransactionByVersionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transactions/by_version/{txn_version}",
        local_var_configuration.base_path,
        txn_version = crate::apis::urlencode(txn_version)
    );
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTransactionByVersionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve on-chain committed transactions. The page size and start ledger version can be provided to get a specific sequence of transactions.  If the version has been pruned, then a 410 will be returned.  To retrieve a pending transaction, use /transactions/by_hash.
pub async fn get_transactions(
    configuration: &configuration::Configuration,
    start: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<models::Transaction>, Error<GetTransactionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/transactions", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTransactionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The output of the transaction will have the exact transaction outputs and events that running an actual signed transaction would have.  However, it will not have the associated state hashes, as they are not updated in storage.  This can be used to estimate the maximum gas units for a submitted transaction.  To use this, you must: - Create a SignedTransaction with a zero-padded signature. - Submit a SubmitTransactionRequest containing a UserTransactionRequest containing that signature.  To use this endpoint with BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs.
pub async fn simulate_transaction(
    configuration: &configuration::Configuration,
    submit_transaction_request: models::SubmitTransactionRequest,
    estimate_max_gas_amount: Option<bool>,
    estimate_gas_unit_price: Option<bool>,
    estimate_prioritized_gas_unit_price: Option<bool>,
) -> Result<Vec<models::UserTransaction>, Error<SimulateTransactionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transactions/simulate",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = estimate_max_gas_amount {
        local_var_req_builder =
            local_var_req_builder.query(&[("estimate_max_gas_amount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = estimate_gas_unit_price {
        local_var_req_builder =
            local_var_req_builder.query(&[("estimate_gas_unit_price", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = estimate_prioritized_gas_unit_price {
        local_var_req_builder = local_var_req_builder.query(&[(
            "estimate_prioritized_gas_unit_price",
            &local_var_str.to_string(),
        )]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&submit_transaction_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SimulateTransactionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This allows you to submit multiple transactions.  The response has three outcomes:  1. All transactions succeed, and it will return a 202 2. Some transactions succeed, and it will return the failed transactions and a 206 3. No transactions succeed, and it will also return the failed transactions and a 206  To submit a transaction as JSON, you must submit a SubmitTransactionRequest. To build this request, do the following:  1. Encode the transaction as BCS. If you are using a language that has native BCS support, make sure to use that library. If not, you may take advantage of /transactions/encode_submission. When using this endpoint, make sure you trust the node you're talking to, as it is possible they could manipulate your request. 2. Sign the encoded transaction and use it to create a TransactionSignature. 3. Submit the request. Make sure to use the \"application/json\" Content-Type.  To submit a transaction as BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs. Make sure to use the `application/x.aptos.signed_transaction+bcs` Content-Type.
pub async fn submit_batch_transactions(
    configuration: &configuration::Configuration,
    submit_transaction_request: Vec<models::SubmitTransactionRequest>,
) -> Result<models::TransactionsBatchSubmissionResult, Error<SubmitBatchTransactionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/transactions/batch", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&submit_transaction_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitBatchTransactionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint accepts transaction submissions in two formats.  To submit a transaction as JSON, you must submit a SubmitTransactionRequest. To build this request, do the following:  1. Encode the transaction as BCS. If you are using a language that has native BCS support, make sure of that library. If not, you may take advantage of /transactions/encode_submission. When using this endpoint, make sure you trust the node you're talking to, as it is possible they could manipulate your request. 2. Sign the encoded transaction and use it to create a TransactionSignature. 3. Submit the request. Make sure to use the \"application/json\" Content-Type.  To submit a transaction as BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs. Make sure to use the `application/x.aptos.signed_transaction+bcs` Content-Type.
pub async fn submit_transaction(
    configuration: &configuration::Configuration,
    submit_transaction_request: models::SubmitTransactionRequest,
) -> Result<models::PendingTransaction, Error<SubmitTransactionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/transactions", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&submit_transaction_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitTransactionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Same as /transactions/by_hash, but will wait for a pending transaction to be committed. To be used as a long poll optimization by clients, to reduce latency caused by polling. The \"long\" poll is generally a second or less but dictated by the server; the client must deal with the result as if the request was a normal /transactions/by_hash request, e.g., by retrying if the transaction is pending.
pub async fn wait_transaction_by_hash(
    configuration: &configuration::Configuration,
    txn_hash: &str,
) -> Result<models::Transaction, Error<WaitTransactionByHashError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transactions/wait_by_hash/{txn_hash}",
        local_var_configuration.base_path,
        txn_hash = crate::apis::urlencode(txn_hash)
    );
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WaitTransactionByHashError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
