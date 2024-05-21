use actix_web::{
    get,
    web::{self, Path},
    HttpResponse,
};

use crate::service::db::Database;

#[get("/claimed/{address}")]
async fn claimed(db: web::Data<Database>, path: Path<String>) -> HttpResponse {
    let address = path.into_inner();
    match db.update_beneficiary(address).await {
        Ok(suc) => match suc {
            Some(success) => HttpResponse::Ok().json(success),
            None => HttpResponse::InternalServerError().body("NOT FOUND".to_string()),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
