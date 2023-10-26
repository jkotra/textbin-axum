mod dblogic;
mod entities;

mod graphql;
use async_graphql::{EmptySubscription, Schema};
use graphql::controllers::{MutationRoot, QueryRoot};
use graphql::{graphiql, graphql_handler};

use utoipa::OpenApi;

mod controllers;
use crate::entities::paste::{PasteCreationResponse, PasteRequest, PasteRequestResponse};
use controllers::controllers::*;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use http::Method;
use log::error;
use tower_http::cors::{Any, CorsLayer};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            get_paste,
            post_paste,
            delete_paste,
            get_latest
        ),
        components(
            schemas(
                PasteRequestResponse,
                PasteCreationResponse,
                PasteRequest
            )
        ),
        tags(
            (name = "textbin-axum", description = "simple text hosting platform.")
        )
    )]
    struct ApiDocs;

    env_logger::init();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDocs::openapi()))
        .layer(Extension(schema))
        .layer(cors);

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
