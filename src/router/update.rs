use std::ops::{Div, Sub};

use actix_web::{
    get,
    web::{self, Path},
    HttpResponse,
};
use solana_transaction_status::{option_serializer::OptionSerializer, EncodedTransaction, UiTransaction};

use crate::{service::db::Database, utils::transaction::{extract_token_balances, get_data_from_transaction}};

#[get("/claimed/{tx}")]
async fn claimed(db: web::Data<Database>, path: Path<String>) -> HttpResponse {
    let signature = path.into_inner();
    println!("sig={signature}");
    let token_mint_address =  dotenv::var("TOKEN_MINT_ADDRESS").unwrap();

    let transaction = get_data_from_transaction(signature).await;

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
                    EncodedTransaction::Json(sg) => {
                      
                       match sg.message {
                           solana_transaction_status::UiMessage::Parsed(_) => todo!(),
                           solana_transaction_status::UiMessage::Raw(h) => {
                            accounts = h;
                            accounts.account_keys.first().unwrap()
                           },
                       }
                       
                    },
                    EncodedTransaction::Accounts(_) => todo!(),
                };
                    
                
                match s {
                    Some(s) => {
                        let pre_token = s.pre_token_balances;
                        let post_token = s.post_token_balances;
                      let pre =  match pre_token {
                            OptionSerializer::Some(s) => {
                              extract_token_balances(s, token_mint_address.clone(), signer.to_string())

                                // HttpResponse::Ok().body("success")
                            },
                            OptionSerializer::None => todo!(),
                            OptionSerializer::Skip => todo!(),
                        
                            
                        };
                        let  post = match post_token {
                            OptionSerializer::Some(st) => {
                                extract_token_balances(st, token_mint_address.clone(), signer.to_string())

                            },
                            OptionSerializer::None => todo!(),
                            OptionSerializer::Skip => todo!(),
                        };
                        
                        let balance = post - pre;
                        println!("token=={}",post.sub(pre));
                        match db.update_beneficiary(signer.to_string(),claimed_time,post.sub(pre).div(1000000000.0)).await {
                            Ok(success) => {
                                println!("success{:?}",success.unwrap());
                                HttpResponse::Ok().body("success")
                            },
                            Err(e) => {HttpResponse::InternalServerError().body(e.to_string())},
                        }

                    },
                    None => todo!(),
                
                    
                
                    
                }
            }
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[get("/get/{address}")]
async fn get_beneficiary(db: web::Data<Database>, path: Path<String>) -> HttpResponse {
    let address = path.into_inner();
    match db.get_beneficiary(address).await {
        Ok(suc) => match suc {
            Some(success) => HttpResponse::Ok().json(success),
            None => HttpResponse::InternalServerError().body("NOT FOUND".to_string()),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
