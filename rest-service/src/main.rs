use std::env;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::{Data, JsonConfig};

use yugabyte::db_connection::CoreDBPool;

use crate::controller::{routes, start_tracing};

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_tracing();
    let core_db_pool_data = Data::new(CoreDBPool::default());
    dotenv::dotenv().expect("Failed to read .env file");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(JsonConfig::default().limit(4096)))
            .app_data(core_db_pool_data.clone())
            .configure(routes)
    })
        .bind(format!("{}:{}", env::var("HOST").unwrap(), env::var("REST_PORT").unwrap()))
        .expect("Server binding exception")
        .run()
        .await
}
