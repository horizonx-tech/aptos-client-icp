# TransactionSignatureAccountSignature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**public_key** | [**models::PublicKey**](PublicKey.md) |  | 
**signature** | [**models::Signature**](Signature.md) |  | 
**public_keys** | [**Vec<models::PublicKey>**](PublicKey.md) |  | 
**signatures** | [**Vec<models::IndexedSignature>**](IndexedSignature.md) |  | 
**threshold** | **i32** | The number of signatures required for a successful transaction | 
**bitmap** | **String** | All bytes (Vec<u8>) data is represented as hex-encoded string prefixed with `0x` and fulfilled with two hex digits per byte.  Unlike the `Address` type, HexEncodedBytes will not trim any zeros.  | 
**signatures_required** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


