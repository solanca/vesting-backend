use actix_cors::Cors;
use actix_web::{middleware::Logger, web::{scope, Data}, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;

use crate::{router::update::{claimed, get_beneficiary}, service::db::Database};

pub mod model;
pub mod service;
pub mod router;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv().ok();
    let db = Database::_init().await;
    let db_data = Data::new(db);

    let server = HttpServer::new(move || {
        App::new().app_data(db_data.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .service(scope("/api").service(claimed).service(get_beneficiary))
        // .app_data(db_data.clone())
    })
    .bind(("0.0.0.0", 5009))?;

    // Log a message indicating that the server is running
    println!("Server is running on port 5009");

    server.run().await
    // println!("Hello, world!");
}
