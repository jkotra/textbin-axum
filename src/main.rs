mod dblogic;
mod entities;

mod graphql;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use graphql::latest::QueryRoot;
use graphql::{graphiql, graphql_handler};

mod controllers;
use controllers::controllers::*;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use http::Method;
use log::error;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    env_logger::init();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let _db_url = std::env::var("DATABASE_URL");

    if _db_url.is_err() {
        error!("please set DATABASE_URL env var!");
        std::process::exit(1);
    }

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api", get(get_paste))
        .route("/api/latest", get(get_latest))
        .route("/api/delete/:uuid", get(delete_paste))
        .route("/api", post(post_paste))
        .route("/api/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
