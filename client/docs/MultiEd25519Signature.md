# MultiEd25519Signature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_keys** | **Vec<String>** | The public keys for the Ed25519 signature | 
**signatures** | **Vec<String>** | Signature associated with the public keys in the same order | 
**threshold** | **i32** | The number of signatures required for a successful transaction | 
**bitmap** | **String** | All bytes (Vec<u8>) data is represented as hex-encoded string prefixed with `0x` and fulfilled with two hex digits per byte.  Unlike the `Address` type, HexEncodedBytes will not trim any zeros.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


