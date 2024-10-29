# \ViewApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view**](ViewApi.md#view) | **POST** /view | Execute view function of a module



## view

> Vec<models::MoveValue> view(view_request, ledger_version)
Execute view function of a module

Execute the Move function with the given parameters and return its execution result.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_request** | [**ViewRequest**](ViewRequest.md) |  | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |

### Return type

[**Vec<models::MoveValue>**](MoveValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x.aptos.view_function+bcs
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

