# MoveModule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | A hex encoded 32 byte Aptos account address.  This is represented in a string as a 64 character hex string, sometimes shortened by stripping leading 0s, and adding a 0x.  For example, address 0x0000000000000000000000000000000000000000000000000000000000000001 is represented as 0x1.  | 
**name** | **String** |  | 
**friends** | **Vec<String>** | Friends of the module | 
**exposed_functions** | [**Vec<models::MoveFunction>**](MoveFunction.md) | Public functions of the module | 
**structs** | [**Vec<models::MoveStruct>**](MoveStruct.md) | Structs of the module | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


