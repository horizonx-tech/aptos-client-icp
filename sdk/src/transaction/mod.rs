use anyhow::Result;
use aptos_client_icp::{
    apis::{
        accounts_api::get_account,
        configuration::Configuration,
        general_api::get_ledger_info,
        transactions_api::{estimate_gas_price, simulate_transaction},
    },
    models::{
        transaction_signature_ed25519_signature::Type, SubmitTransactionRequest,
        TransactionPayload, TransactionSignature, TransactionSignatureEd25519Signature,
    },
};
use serde::{Deserialize, Serialize};

use crate::signer::{SchnorrKeyIds, Signer, ThresholdSigner};

pub struct TransactionBuilder {
    payload: TransactionPayload,
}
/// RawTransaction is the portion of a transaction that a client signs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawTransaction {
    /// Sender's address.
    sender: String,

    /// Sequence number of this transaction. This must match the sequence number
    /// stored in the sender's account at the time the transaction executes.
    sequence_number: u64,

    /// The transaction payload, e.g., a script to execute.
    payload: TransactionPayload,

    /// Maximal total gas to spend for this transaction.
    max_gas_amount: u64,

    /// Price to be paid per gas unit.
    gas_unit_price: u64,

    /// Expiration timestamp for this transaction, represented
    /// as seconds from the Unix Epoch. If the current blockchain timestamp
    /// is greater than or equal to this time, then the transaction has
    /// expired and will be discarded. This can be set to a large value far
    /// in the future to indicate that a transaction does not expire.
    expiration_timestamp_secs: u64,

    /// Chain ID of the Aptos network this transaction is intended for.
    chain_id: u8,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            payload: TransactionPayload::default(),
        }
    }
    pub fn payload(mut self, payload: TransactionPayload) -> Self {
        self.payload = payload;
        self
    }
    pub async fn build(self, signer: ThresholdSigner) -> Result<SubmitTransactionRequest> {
        let sequence_number = get_account(
            &Configuration::default(),
            signer.try_pubkey()?.as_str(),
            None,
        )
        .await?
        .sequence_number;
        let ledger_info = get_ledger_info(&Configuration::default()).await?;
        let gas_unit_price = estimate_gas_price(&Configuration::default())
            .await?
            .gas_estimate;
        // nano to sec
        let now = ic_cdk::api::time() / 1_000_000_000;
        let expiration_time_secs = now + 1800;
        let raw_tx = RawTransaction {
            sender: signer.try_pubkey()?,
            sequence_number: sequence_number.parse()?,
            payload: self.payload.clone(),
            max_gas_amount: 100_000,
            gas_unit_price: gas_unit_price as u64,
            expiration_timestamp_secs: expiration_time_secs,
            chain_id: ledger_info.chain_id as u8,
        };
        let signer = ThresholdSigner::new(SchnorrKeyIds::TestKeyLocalDevelopment).await?;
        let signature = signer.try_sign_message(&raw_tx).await?;
        let txns = simulate_transaction(
            &Configuration::default(),
            SubmitTransactionRequest {
                expiration_timestamp_secs: expiration_time_secs.to_string(),
                gas_unit_price: gas_unit_price.to_string(),
                sender: signer.try_pubkey()?,
                payload: Box::new(self.payload.clone()),
                sequence_number: sequence_number.to_string(),
                signature: Box::new(TransactionSignature::Ed25519Signature(Box::new(
                    TransactionSignatureEd25519Signature {
                        public_key: signer.try_pubkey()?,
                        signature: format!("0x{}", hex::encode(&signature)),
                        r#type: Type::Ed25519Signature,
                    },
                ))),
                max_gas_amount: "".to_string(),
            },
            Some(true),
            Some(true),
            Some(true),
        )
        .await?;
        let simulated_txn = txns.first().unwrap();
        let max_possible_gas = simulated_txn.max_gas_amount.clone();
        let transaction_signature = TransactionSignature::Ed25519Signature(Box::new(
            TransactionSignatureEd25519Signature {
                public_key: signer.try_pubkey()?,
                signature: format!("0x{}", hex::encode(&signature)),
                r#type: Type::Ed25519Signature,
            },
        ));
        let request = SubmitTransactionRequest {
            expiration_timestamp_secs: expiration_time_secs.to_string(),
            gas_unit_price: gas_unit_price.to_string(),
            sender: signer.try_pubkey()?,
            max_gas_amount: max_possible_gas,
            payload: Box::new(self.payload),
            sequence_number: sequence_number.to_string(),
            signature: Box::new(transaction_signature),
        };
        Ok(request)
    }
}
