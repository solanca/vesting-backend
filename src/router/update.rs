use std::{
    ops::{Div, Sub},
    thread::sleep,
    time::Duration,
};

use crate::{
    model::beneficiaries_model::SubmitTransactionRequest,
    utils::transaction::{extract_token_balances, get_data_from_transaction},
    AppState,
};
use actix_web::{
    get, post,
    web::{self, Path},
    HttpResponse, Responder,
};
use base64::decode;
use solana_client::rpc_response::RpcSimulateTransactionResult;
// use base64::decode;
use solana_sdk::transaction::Transaction;
// use solana_sdk::short_vec::deserialize;
use solana_transaction_status::{option_serializer::OptionSerializer, EncodedTransaction};

#[get("/get/{address}")]
async fn get_beneficiary(db: web::Data<AppState>, path: Path<String>) -> HttpResponse {
    let address = path.into_inner();
    match db.db.get_beneficiary(address).await {
        Ok(suc) => match suc {
            Some(success) => HttpResponse::Ok().json(success),
            None => HttpResponse::InternalServerError().body("NOT FOUND".to_string()),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/submit-transaction")]
async fn submit_transaction(
    data: web::Data<AppState>,
    req: web::Json<SubmitTransactionRequest>,
) -> impl Responder {
    let transaction_data = match decode(&req.transaction) {
        Ok(data) => data,
        Err(_) => return HttpResponse::BadRequest().body("Invalid transaction data"),
    };

    let transaction: Transaction = match bincode::deserialize(&transaction_data) {
        Ok(tx) => tx,
        Err(_) => return HttpResponse::BadRequest().body("Failed to deserialize transaction"),
    };

    let app_state = data.clone();

    let result = web::block(move || {
        let tx = transaction.clone();
        println!("sleep===");
        // sleep(Duration::from_secs(10));
        println!("sleep===>>>>>>>");
        app_state.rpc_client.send_and_confirm_transaction(&tx)
    })
    .await;

    match result {
        Ok(tx_signature) => {
            let token_mint_address = dotenv::var("TOKEN_MINT_ADDRESS").unwrap();
            match tx_signature {
                Ok(signature) => {
                    let transaction = get_data_from_transaction(signature.to_string()).await;

                    match transaction {
                        Ok(tr) => match tr {
                            Ok(t) => {
                                let claimed_time = t.block_time.unwrap();

                                let s = t.transaction.meta;
                                let sign = t.transaction.transaction;
                                let accounts;
                                let signer = match sign {
                                    EncodedTransaction::LegacyBinary(_) => todo!(),
                                    EncodedTransaction::Binary(_, _) => todo!(),
                                    EncodedTransaction::Json(sg) => match sg.message {
                                        solana_transaction_status::UiMessage::Parsed(_) => todo!(),
                                        solana_transaction_status::UiMessage::Raw(h) => {
                                            accounts = h;
                                            accounts.account_keys.first().unwrap()
                                        }
                                    },
                                    EncodedTransaction::Accounts(_) => todo!(),
                                };

                                match s {
                                    Some(s) => {
                                        let pre_token = s.pre_token_balances;
                                        let post_token = s.post_token_balances;
                                        let pre = match pre_token {
                                            OptionSerializer::Some(s) => {
                                                extract_token_balances(
                                                    s,
                                                    token_mint_address.clone(),
                                                    signer.to_string(),
                                                )

                                                // HttpResponse::Ok().body("success")
                                            }
                                            OptionSerializer::None => todo!(),
                                            OptionSerializer::Skip => todo!(),
                                        };
                                        let post = match post_token {
                                            OptionSerializer::Some(st) => extract_token_balances(
                                                st,
                                                token_mint_address.clone(),
                                                signer.to_string(),
                                            ),
                                            OptionSerializer::None => todo!(),
                                            OptionSerializer::Skip => todo!(),
                                        };

                                        println!("token=={}", post.sub(pre));
                                        match data
                                            .db
                                            .update_beneficiary(
                                                signer.to_string(),
                                                claimed_time,
                                                post.sub(pre).div(1000000000.0),
                                            )
                                            .await
                                        {
                                            Ok(success) => {
                                                println!("success{:?}", success.unwrap());
                                                HttpResponse::Ok().body("success")
                                            }
                                            Err(e) => HttpResponse::InternalServerError()
                                                .body(e.to_string()),
                                        }
                                    }
                                    None => todo!(),
                                }
                            }
                            Err(e) => {
                                return HttpResponse::InternalServerError().body(e.to_string())
                            }
                        },
                        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
                    }
                }
                Err(e) => {
                    if let solana_client::client_error::ClientErrorKind::RpcError(rpc_error) =
                        &e.kind
                    {
                        if let solana_client::rpc_request::RpcError::RpcResponseError {
                            data, ..
                        } = rpc_error
                        {
                            if let solana_client::rpc_request::RpcResponseErrorData::SendTransactionPreflightFailure(RpcSimulateTransactionResult { logs, .. }) = data {
                                if let Some(logs) = logs {
                                    for log in logs {
                                        if log.contains("Error Code") {
                                            return HttpResponse::InternalServerError()
                                                .body(log.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // println!("kind=={:?}", e);

                    return HttpResponse::InternalServerError().body(e.to_string());
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
