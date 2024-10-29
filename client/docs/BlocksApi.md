# \BlocksApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_by_height**](BlocksApi.md#get_block_by_height) | **GET** /blocks/by_height/{block_height} | Get blocks by height
[**get_block_by_version**](BlocksApi.md#get_block_by_version) | **GET** /blocks/by_version/{version} | Get blocks by version



## get_block_by_height

> models::Block get_block_by_height(block_height, with_transactions)
Get blocks by height

This endpoint allows you to get the transactions in a block and the corresponding block information.  Transactions are limited by max default transactions size.  If not all transactions are present, the user will need to query for the rest of the transactions via the get transactions API.  If the block is pruned, it will return a 410

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_height** | **i32** | Block height to lookup.  Starts at 0 | [required] |
**with_transactions** | Option<**bool**> | If set to true, include all transactions in the block  If not provided, no transactions will be retrieved |  |

### Return type

[**models::Block**](Block.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_by_version

> models::Block get_block_by_version(version, with_transactions)
Get blocks by version

This endpoint allows you to get the transactions in a block and the corresponding block information given a version in the block.  Transactions are limited by max default transactions size.  If not all transactions are present, the user will need to query for the rest of the transactions via the get transactions API.  If the block has been pruned, it will return a 410

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **i32** | Ledger version to lookup block information for. | [required] |
**with_transactions** | Option<**bool**> | If set to true, include all transactions in the block  If not provided, no transactions will be retrieved |  |

### Return type

[**models::Block**](Block.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

