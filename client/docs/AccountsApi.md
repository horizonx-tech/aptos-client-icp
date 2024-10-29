# \AccountsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account**](AccountsApi.md#get_account) | **GET** /accounts/{address} | Get account
[**get_account_module**](AccountsApi.md#get_account_module) | **GET** /accounts/{address}/module/{module_name} | Get account module
[**get_account_modules**](AccountsApi.md#get_account_modules) | **GET** /accounts/{address}/modules | Get account modules
[**get_account_resource**](AccountsApi.md#get_account_resource) | **GET** /accounts/{address}/resource/{resource_type} | Get account resource
[**get_account_resources**](AccountsApi.md#get_account_resources) | **GET** /accounts/{address}/resources | Get account resources



## get_account

> models::AccountData get_account(address, ledger_version)
Get account

Return the authentication key and the sequence number for an account address. Optionally, a ledger version can be specified. If the ledger version is not specified in the request, the latest ledger version is used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account with or without a `0x` prefix | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |

### Return type

[**models::AccountData**](AccountData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_module

> models::MoveModuleBytecode get_account_module(address, module_name, ledger_version)
Get account module

Retrieves an individual module from a given account and at a specific ledger version. If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account with or without a `0x` prefix | [required] |
**module_name** | **String** | Name of module to retrieve e.g. `coin` | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |

### Return type

[**models::MoveModuleBytecode**](MoveModuleBytecode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_modules

> Vec<models::MoveModuleBytecode> get_account_modules(address, ledger_version, start, limit)
Get account modules

Retrieves all account modules' bytecode for a given account at a specific ledger version. If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account with or without a `0x` prefix | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |
**start** | Option<**String**> | Cursor specifying where to start for pagination  This cursor cannot be derived manually client-side. Instead, you must call this endpoint once without this query parameter specified, and then use the cursor returned in the X-Aptos-Cursor header in the response. |  |
**limit** | Option<**i32**> | Max number of account modules to retrieve  If not provided, defaults to default page size. |  |

### Return type

[**Vec<models::MoveModuleBytecode>**](MoveModuleBytecode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_resource

> models::MoveResource get_account_resource(address, resource_type, ledger_version)
Get account resource

Retrieves an individual resource from a given account and at a specific ledger version. If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account with or without a `0x` prefix | [required] |
**resource_type** | **String** | Name of struct to retrieve e.g. `0x1::account::Account` | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |

### Return type

[**models::MoveResource**](MoveResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_resources

> Vec<models::MoveResource> get_account_resources(address, ledger_version, start, limit)
Get account resources

Retrieves all account resources for a given account and a specific ledger version.  If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account with or without a `0x` prefix | [required] |
**ledger_version** | Option<**String**> | Ledger version to get state of account  If not provided, it will be the latest version |  |
**start** | Option<**String**> | Cursor specifying where to start for pagination  This cursor cannot be derived manually client-side. Instead, you must call this endpoint once without this query parameter specified, and then use the cursor returned in the X-Aptos-Cursor header in the response. |  |
**limit** | Option<**i32**> | Max number of account resources to retrieve  If not provided, defaults to default page size. |  |

### Return type

[**Vec<models::MoveResource>**](MoveResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

