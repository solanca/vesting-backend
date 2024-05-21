use std::str::FromStr;

use actix_web::web;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};

pub async fn get_data_from_transaction(signature:String) -> Result<Result<EncodedConfirmedTransactionWithStatusMeta, solana_client::client_error::ClientError>, actix_web::error::BlockingError> {
    let sign = Signature::from_str(&signature).unwrap();
    web::block(move || {
        let client = RpcClient::new("https://api.devnet.solana.com");
        // let signature = SolSignature::from_str(&result._id.clone()).unwrap();
        let config = RpcTransactionConfig {
            encoding: None,
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        };
        client.get_transaction_with_config(&sign, config)
    }).await

}