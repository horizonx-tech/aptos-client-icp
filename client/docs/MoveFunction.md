# MoveFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**visibility** | [**models::MoveFunctionVisibility**](MoveFunctionVisibility.md) |  | 
**is_entry** | **bool** | Whether the function can be called as an entry function directly in a transaction | 
**is_view** | **bool** | Whether the function is a view function or not | 
**generic_type_params** | [**Vec<models::MoveFunctionGenericTypeParam>**](MoveFunctionGenericTypeParam.md) | Generic type params associated with the Move function | 
**params** | **Vec<String>** | Parameters associated with the move function | 
**r#return** | **Vec<String>** | Return type of the function | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


