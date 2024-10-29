# MultisigTransactionPayloadEntryFunctionPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**function** | **String** | Entry function id is string representation of a entry function defined on-chain.  Format: `{address}::{module name}::{function name}`  Both `module name` and `function name` are case-sensitive.  | 
**type_arguments** | **Vec<String>** | Type arguments of the function | 
**arguments** | [**Vec<serde_json::Value>**](serde_json::Value.md) | Arguments of the function | 
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


