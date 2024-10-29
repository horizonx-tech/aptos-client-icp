# \TransactionsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**encode_submission**](TransactionsApi.md#encode_submission) | **POST** /transactions/encode_submission | Encode submission
[**estimate_gas_price**](TransactionsApi.md#estimate_gas_price) | **GET** /estimate_gas_price | Estimate gas price
[**get_account_transactions**](TransactionsApi.md#get_account_transactions) | **GET** /accounts/{address}/transactions | Get account transactions
[**get_transaction_by_hash**](TransactionsApi.md#get_transaction_by_hash) | **GET** /transactions/by_hash/{txn_hash} | Get transaction by hash
[**get_transaction_by_version**](TransactionsApi.md#get_transaction_by_version) | **GET** /transactions/by_version/{txn_version} | Get transaction by version
[**get_transactions**](TransactionsApi.md#get_transactions) | **GET** /transactions | Get transactions
[**simulate_transaction**](TransactionsApi.md#simulate_transaction) | **POST** /transactions/simulate | Simulate transaction
[**submit_batch_transactions**](TransactionsApi.md#submit_batch_transactions) | **POST** /transactions/batch | Submit batch transactions
[**submit_transaction**](TransactionsApi.md#submit_transaction) | **POST** /transactions | Submit transaction
[**wait_transaction_by_hash**](TransactionsApi.md#wait_transaction_by_hash) | **GET** /transactions/wait_by_hash/{txn_hash} | Wait for transaction by hash



## encode_submission

> String encode_submission(encode_submission_request)
Encode submission

This endpoint accepts an EncodeSubmissionRequest, which internally is a UserTransactionRequestInner (and optionally secondary signers) encoded as JSON, validates the request format, and then returns that request encoded in BCS. The client can then use this to create a transaction signature to be used in a SubmitTransactionRequest, which it then passes to the /transactions POST endpoint.  To be clear, this endpoint makes it possible to submit transaction requests to the API from languages that do not have library support for BCS. If you are using an SDK that has BCS support, such as the official Rust, TypeScript, or Python SDKs, you do not need to use this endpoint.  To sign a message using the response from this endpoint: - Decode the hex encoded string in the response to bytes. - Sign the bytes to create the signature. - Use that as the signature field in something like Ed25519Signature, which you then use to build a TransactionSignature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**encode_submission_request** | [**EncodeSubmissionRequest**](EncodeSubmissionRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## estimate_gas_price

> models::GasEstimation estimate_gas_price()
Estimate gas price

Gives an estimate of the gas unit price required to get a transaction on chain in a reasonable amount of time. The gas unit price is the amount that each transaction commits to pay for each unit of gas consumed in executing the transaction. The estimate is based on recent history: it gives the minimum gas that would have been required to get into recent blocks, for blocks that were full. (When blocks are not full, the estimate will match the minimum gas unit price.)  The estimation is given in three values: de-prioritized (low), regular, and prioritized (aggressive). Using a more aggressive value increases the likelihood that the transaction will make it into the next block; more aggressive values are computed with a larger history and higher percentile statistics. More details are in AIP-34.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GasEstimation**](GasEstimation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_transactions

> Vec<models::Transaction> get_account_transactions(address, start, limit)
Get account transactions

Retrieves on-chain committed transactions from an account. If the start version is too far in the past, a 410 will be returned.  If no start version is given, it will start at version 0.  To retrieve a pending transaction, use /transactions/by_hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account with or without a `0x` prefix | [required] |
**start** | Option<**String**> | Account sequence number to start list of transactions  If not provided, defaults to showing the latest transactions |  |
**limit** | Option<**i32**> | Max number of transactions to retrieve.  If not provided, defaults to default page size |  |

### Return type

[**Vec<models::Transaction>**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_by_hash

> models::Transaction get_transaction_by_hash(txn_hash)
Get transaction by hash

Look up a transaction by its hash. This is the same hash that is returned by the API when submitting a transaction (see PendingTransaction).  When given a transaction hash, the server first looks for the transaction in storage (on-chain, committed). If no on-chain transaction is found, it looks the transaction up by hash in the mempool (pending, not yet committed).  To create a transaction hash by yourself, do the following: 1. Hash message bytes: \"RawTransaction\" bytes + BCS bytes of [Transaction](https://aptos-labs.github.io/aptos-core/aptos_types/transaction/enum.Transaction.html). 2. Apply hash algorithm `SHA3-256` to the hash message bytes. 3. Hex-encode the hash bytes with `0x` prefix.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txn_hash** | **String** | Hash of transaction to retrieve | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_by_version

> models::Transaction get_transaction_by_version(txn_version)
Get transaction by version

Retrieves a transaction by a given version. If the version has been pruned, a 410 will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txn_version** | **String** | Version of transaction to retrieve | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> Vec<models::Transaction> get_transactions(start, limit)
Get transactions

Retrieve on-chain committed transactions. The page size and start ledger version can be provided to get a specific sequence of transactions.  If the version has been pruned, then a 410 will be returned.  To retrieve a pending transaction, use /transactions/by_hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**String**> | Ledger version to start list of transactions  If not provided, defaults to showing the latest transactions |  |
**limit** | Option<**i32**> | Max number of transactions to retrieve.  If not provided, defaults to default page size |  |

### Return type

[**Vec<models::Transaction>**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## simulate_transaction

> Vec<models::UserTransaction> simulate_transaction(submit_transaction_request, estimate_max_gas_amount, estimate_gas_unit_price, estimate_prioritized_gas_unit_price)
Simulate transaction

The output of the transaction will have the exact transaction outputs and events that running an actual signed transaction would have.  However, it will not have the associated state hashes, as they are not updated in storage.  This can be used to estimate the maximum gas units for a submitted transaction.  To use this, you must: - Create a SignedTransaction with a zero-padded signature. - Submit a SubmitTransactionRequest containing a UserTransactionRequest containing that signature.  To use this endpoint with BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_transaction_request** | [**SubmitTransactionRequest**](SubmitTransactionRequest.md) |  | [required] |
**estimate_max_gas_amount** | Option<**bool**> | If set to true, the max gas value in the transaction will be ignored and the maximum possible gas will be used |  |
**estimate_gas_unit_price** | Option<**bool**> | If set to true, the gas unit price in the transaction will be ignored and the estimated value will be used |  |
**estimate_prioritized_gas_unit_price** | Option<**bool**> | If set to true, the transaction will use a higher price than the original estimate. |  |

### Return type

[**Vec<models::UserTransaction>**](UserTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x.aptos.signed_transaction+bcs
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_batch_transactions

> models::TransactionsBatchSubmissionResult submit_batch_transactions(submit_transaction_request)
Submit batch transactions

This allows you to submit multiple transactions.  The response has three outcomes:  1. All transactions succeed, and it will return a 202 2. Some transactions succeed, and it will return the failed transactions and a 206 3. No transactions succeed, and it will also return the failed transactions and a 206  To submit a transaction as JSON, you must submit a SubmitTransactionRequest. To build this request, do the following:  1. Encode the transaction as BCS. If you are using a language that has native BCS support, make sure to use that library. If not, you may take advantage of /transactions/encode_submission. When using this endpoint, make sure you trust the node you're talking to, as it is possible they could manipulate your request. 2. Sign the encoded transaction and use it to create a TransactionSignature. 3. Submit the request. Make sure to use the \"application/json\" Content-Type.  To submit a transaction as BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs. Make sure to use the `application/x.aptos.signed_transaction+bcs` Content-Type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_transaction_request** | [**Vec<models::SubmitTransactionRequest>**](SubmitTransactionRequest.md) |  | [required] |

### Return type

[**models::TransactionsBatchSubmissionResult**](TransactionsBatchSubmissionResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x.aptos.signed_transaction+bcs
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_transaction

> models::PendingTransaction submit_transaction(submit_transaction_request)
Submit transaction

This endpoint accepts transaction submissions in two formats.  To submit a transaction as JSON, you must submit a SubmitTransactionRequest. To build this request, do the following:  1. Encode the transaction as BCS. If you are using a language that has native BCS support, make sure of that library. If not, you may take advantage of /transactions/encode_submission. When using this endpoint, make sure you trust the node you're talking to, as it is possible they could manipulate your request. 2. Sign the encoded transaction and use it to create a TransactionSignature. 3. Submit the request. Make sure to use the \"application/json\" Content-Type.  To submit a transaction as BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs. Make sure to use the `application/x.aptos.signed_transaction+bcs` Content-Type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_transaction_request** | [**SubmitTransactionRequest**](SubmitTransactionRequest.md) |  | [required] |

### Return type

[**models::PendingTransaction**](PendingTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x.aptos.signed_transaction+bcs
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wait_transaction_by_hash

> models::Transaction wait_transaction_by_hash(txn_hash)
Wait for transaction by hash

Same as /transactions/by_hash, but will wait for a pending transaction to be committed. To be used as a long poll optimization by clients, to reduce latency caused by polling. The \"long\" poll is generally a second or less but dictated by the server; the client must deal with the result as if the request was a normal /transactions/by_hash request, e.g., by retrying if the transaction is pending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txn_hash** | **String** | Hash of transaction to retrieve | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

