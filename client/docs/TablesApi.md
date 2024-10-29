# \TablesApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_raw_table_item**](TablesApi.md#get_raw_table_item) | **POST** /tables/{table_handle}/raw_item | Get raw table item
[**get_table_item**](TablesApi.md#get_table_item) | **POST** /tables/{table_handle}/item | Get table item



## get_raw_table_item

> models::MoveValue get_raw_table_item(table_handle, raw_table_item_request, ledger_version)
Get raw table item

Get a table item at a specific ledger version from the table identified by {table_handle} in the path and the \"key\" (RawTableItemRequest) provided in the request body.  The `get_raw_table_item` requires only a serialized key comparing to the full move type information comparing to the `get_table_item` api, and can only return the query in the bcs format.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_handle** | **String** | Table handle hex encoded 32-byte string | [required] |
**raw_table_item_request** | [**RawTableItemRequest**](RawTableItemRequest.md) |  | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |

### Return type

[**models::MoveValue**](MoveValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_table_item

> models::MoveValue get_table_item(table_handle, table_item_request, ledger_version)
Get table item

Get a table item at a specific ledger version from the table identified by {table_handle} in the path and the \"key\" (TableItemRequest) provided in the request body.  This is a POST endpoint because the \"key\" for requesting a specific table item (TableItemRequest) could be quite complex, as each of its fields could themselves be composed of other structs. This makes it impractical to express using query params, meaning GET isn't an option.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_handle** | **String** | Table handle hex encoded 32-byte string | [required] |
**table_item_request** | [**TableItemRequest**](TableItemRequest.md) |  | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |

### Return type

[**models::MoveValue**](MoveValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

