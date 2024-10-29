# TransactionSignatureFeePayerSignature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sender** | [**models::AccountSignature**](AccountSignature.md) |  | 
**secondary_signer_addresses** | **Vec<String>** | The other involved parties' addresses | 
**secondary_signers** | [**Vec<models::AccountSignature>**](AccountSignature.md) | The associated signatures, in the same order as the secondary addresses | 
**fee_payer_address** | **String** |  | 
**fee_payer_signer** | [**models::FeePayerSignatureFeePayerSigner**](FeePayerSignature_fee_payer_signer.md) |  | 
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


