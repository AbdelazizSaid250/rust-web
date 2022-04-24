use std::io;

use actix_web::{App, HttpServer, middleware};
use actix_web::web::{Data, JsonConfig};

use yugabyte::db_connection::CoreDBPool;

use crate::gql::{logging_setup, routes};

mod gql;

#[actix_web::main]
async fn main() -> io::Result<()> {

    logging_setup();

    // Instantiate a new connection pool
    let core_db_pool_data = Data::new(CoreDBPool::default().0);

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(JsonConfig::default().limit(4096)))
            .app_data(core_db_pool_data.clone())
            .wrap(middleware::Logger::default())
            .configure(routes)
    })
        .bind("127.0.0.1:3001")?
        .run()
        .await
}
