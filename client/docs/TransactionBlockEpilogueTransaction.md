# TransactionBlockEpilogueTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **String** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**hash** | **String** |  | 
**state_change_hash** | **String** |  | 
**event_root_hash** | **String** |  | 
**state_checkpoint_hash** | Option<**String**> |  | [optional]
**gas_used** | **String** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**success** | **bool** | Whether the transaction was successful | 
**vm_status** | **String** | The VM status of the transaction, can tell useful information in a failure | 
**accumulator_root_hash** | **String** |  | 
**changes** | [**Vec<models::WriteSetChange>**](WriteSetChange.md) | Final state of resources changed by the transaction | 
**timestamp** | **String** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**block_end_info** | Option<[**models::BlockEndInfo**](BlockEndInfo.md)> |  | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


