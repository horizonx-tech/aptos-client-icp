use aptos_client_icp::apis::{accounts_api::get_account, configuration::Configuration};
use aptos_sdk_icp::signer::{Signer, ThresholdSigner};

#[ic_cdk::update]
async fn get_pubkey() -> String {
    let signer =
        ThresholdSigner::new(aptos_sdk_icp::signer::SchnorrKeyIds::TestKeyLocalDevelopment)
            .await
            .unwrap();
    let pubkey = signer.try_pubkey().unwrap();
    pubkey.to_string()
}

#[ic_cdk::update]
async fn get_authentication_key() -> String {
    let ac = get_account(&Configuration::default(), &get_pubkey().await, None).await;
    if let Ok(ac) = ac {
        ac.authentication_key.to_string()
    } else {
        let err = ac.unwrap_err();
        err.to_string()
    }
}
