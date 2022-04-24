use std::env;

use actix_web::{Error, HttpResponse, web};
use actix_web::web::Data;
use juniper::http::GraphQLRequest;

use yugabyte::context::GraphQLContext;
use yugabyte::db_connection::PgPool;

use crate::gql::schema::auth_user_schema::{auth_user_schema, AuthUserSchema};
use crate::gql::schema::member_schema::{member_schema, MemberSchema};

mod schema;

pub fn routes(config: &mut web::ServiceConfig) {
    let auth_schema = Data::new(auth_user_schema());
    let member_schema = Data::new(member_schema());
    config
        .app_data(auth_schema)
        .app_data(member_schema)
        .route("/graphql", web::post().to(auth_user_graphql))
        .route("/graphql", web::post().to(member_graphql));
}

// The core handler that provides all GraphQL functionality.
async fn auth_user_graphql(
    // The DB connection pool
    pool: web::Data<PgPool>,
    // The GraphQL schema
    schema: web::Data<AuthUserSchema>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // Instantiate a context
    let context = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };

    // Handle the incoming request and return a string result (or error)
    let res = web::block(move || {
        let graphql_response = data.execute_sync(&schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&graphql_response)?)
    })
        .await
        .map_err(Error::from)??;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

async fn member_graphql(
    // The DB connection pool
    pool: web::Data<PgPool>,
    // The GraphQL schema
    schema: web::Data<MemberSchema>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // Instantiate a context
    let context = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };

    // Handle the incoming request and return a string result (or error)
    let res = web::block(move || {
        let res = data.execute_sync(&schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await
        .map_err(Error::from)??;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

pub(crate) fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
