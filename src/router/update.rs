use actix_web::{
    get,
    web::{self, Path},
    HttpResponse,
};

use crate::{service::db::Database, utils::transaction::get_data_from_transaction};

#[get("/claimed/{tx}")]
async fn claimed(db: web::Data<Database>, path: Path<String>) -> HttpResponse {
    let signature = path.into_inner();
    println!("sig={signature}");

    let transaction = get_data_from_transaction(signature).await;

    match transaction {
        Ok(tr) => match tr {
            Ok(t) => {
                // println!("t=={:?}",t);
                let claimed_time = t.block_time.unwrap();

                let s = t.transaction.meta;
                match s {
                    Some(s) => {
                        let pre_token = s.pre_token_balances;
                        let post_token = s.post_token_balances;
                      let pre =  match pre_token {
                            solana_transaction_status::option_serializer::OptionSerializer::Some(s) => {
                               match s.last() {
                                   Some(s) => {
                                    // println!("uitoken={}",s.ui_token_amount.ui_amount.unwrap());
                                    s.ui_token_amount.ui_amount.unwrap()
                                   },
                                   None => {0.0},
                               }

                                // HttpResponse::Ok().body("success")
                            },
                            solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
                            solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(),
                        
                            
                        };
                        let (owner, post) = match post_token {
                            solana_transaction_status::option_serializer::OptionSerializer::Some(st) => {
                                match st.last() {
                                    Some(s) => {
                                        // Clone the necessary data from `s` before the borrow ends.
                                        let ui_token_amount = s.ui_token_amount.ui_amount.unwrap();
                                        let owner = match &s.owner {
                                            solana_transaction_status::option_serializer::OptionSerializer::Some(o) => o.clone(),
                                            solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
                                            solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(),
                                        };
                                        (owner, ui_token_amount)
                                    },
                                    None => todo!(),
                                }
                            },
                            solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
                            solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(),
                        };
                        
                        let balance = post - pre;
                        println!("token=={}",balance.abs());
                        match db.update_beneficiary(owner.to_string(),claimed_time,balance).await {
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
