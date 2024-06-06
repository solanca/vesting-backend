use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use router::update::submit_transaction;
use solana_client::rpc_client::RpcClient;

use crate::{router::update::get_beneficiary, service::db::Database};

pub mod model;
pub mod router;
pub mod service;
pub mod utils;

pub struct AppState {
    pub rpc_client: Arc<RpcClient>,
    pub db: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv().ok();
    let db = Database::_init().await;
    // let db_data = Data::new(db);
    let solana_rpc = dotenv::var("SOLANA_RPC_URL").unwrap();
    let rpc_client = Arc::new(RpcClient::new(solana_rpc));

    let app_state = Data::new(AppState { rpc_client, db });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .service(
                scope("/api")
                    .service(get_beneficiary)
                    .service(submit_transaction),
            )
    })
    .bind(("0.0.0.0", 5009))?;

    // Log a message indicating that the server is running
    println!("Server is running on port 5009");

    server.run().await
    // println!("Hello, world!");
}
