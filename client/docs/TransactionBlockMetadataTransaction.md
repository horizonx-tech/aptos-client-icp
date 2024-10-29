# TransactionBlockMetadataTransaction

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
**id** | **String** |  | 
**epoch** | **String** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**round** | **String** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**events** | [**Vec<models::Event>**](Event.md) | The events emitted at the block creation | 
**previous_block_votes_bitvec** | **Vec<i32>** | Previous block votes | 
**proposer** | **String** | A hex encoded 32 byte Aptos account address.  This is represented in a string as a 64 character hex string, sometimes shortened by stripping leading 0s, and adding a 0x.  For example, address 0x0000000000000000000000000000000000000000000000000000000000000001 is represented as 0x1.  | 
**failed_proposer_indices** | **Vec<i32>** | The indices of the proposers who failed to propose | 
**timestamp** | **String** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**block_metadata_extension** | Option<[**models::BlockMetadataTransactionBlockMetadataExtension**](BlockMetadataTransaction_block_metadata_extension.md)> |  | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


