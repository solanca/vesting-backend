use std::str::FromStr;

use actix_web::web;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding, UiTransactionTokenBalance};

pub async fn get_data_from_transaction(signature:String) -> Result<Result<EncodedConfirmedTransactionWithStatusMeta, solana_client::client_error::ClientError>, actix_web::error::BlockingError> {
    let sign = Signature::from_str(&signature).unwrap();
    let solana_rpc = dotenv::var("SOLANA_RPC_URL").unwrap();
    web::block(move || {
        let client = RpcClient::new(solana_rpc);
        // let signature = SolSignature::from_str(&result._id.clone()).unwrap();
        let config = RpcTransactionConfig {
            encoding: None,
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        };
        client.get_transaction_with_config(&sign, config)
    }).await

}

pub fn extract_token_balances(balances:Vec<UiTransactionTokenBalance>,token_mint_address:String,owner:String) ->f64 {
    balances.iter().filter(|balance| {
        let tx_owner = match &balance.owner {
            solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
            solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
            solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(),
        };

        return balance.mint == token_mint_address && *tx_owner == owner;

    }).map(|balance| balance.ui_token_amount.amount.parse::<f64>().unwrap_or(0.0)).sum()
}