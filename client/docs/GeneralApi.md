# \GeneralApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ledger_info**](GeneralApi.md#get_ledger_info) | **GET** / | Get ledger info
[**healthy**](GeneralApi.md#healthy) | **GET** /-/healthy | Check basic node health
[**info**](GeneralApi.md#info) | **GET** /info | Show some basic info of the node.
[**spec**](GeneralApi.md#spec) | **GET** /spec | Show OpenAPI explorer



## get_ledger_info

> models::IndexResponse get_ledger_info()
Get ledger info

Get the latest ledger information, including data such as chain ID, role type, ledger versions, epoch, etc.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IndexResponse**](IndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## healthy

> models::HealthCheckSuccess healthy(duration_secs)
Check basic node health

By default this endpoint just checks that it can get the latest ledger info and then returns 200.  If the duration_secs param is provided, this endpoint will return a 200 if the following condition is true:  `server_latest_ledger_info_timestamp >= server_current_time_timestamp - duration_secs`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**duration_secs** | Option<**i32**> | Threshold in seconds that the server can be behind to be considered healthy  If not provided, the healthcheck will always succeed |  |

### Return type

[**models::HealthCheckSuccess**](HealthCheckSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> std::collections::HashMap<String, serde_json::Value> info()
Show some basic info of the node.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spec

> String spec()
Show OpenAPI explorer

Provides a UI that you can use to explore the API. You can also retrieve the API directly at `/spec.yaml` and `/spec.json`.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

