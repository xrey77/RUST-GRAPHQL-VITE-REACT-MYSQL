// run : cargo run
// graphql endpoint : http://127.0.0.1:3000/graphiql

pub mod graphql; // This tells Rust to look for src/graphql/mod.rs
pub mod schema;
mod core;
mod routes;

use tower_http::cors::{Any, CorsLayer};
use axum::{routing::{get, post}, Router};
use askama::Template;
use axum::response::Html;
use axum::http::{self, Method};
use crate::routes::graphiql::{graphiql_handler, graphiql_source};
use tower_http::services::ServeDir;
use crate::core::database::establish_connection;

#[tokio::main]
async fn main() {
    let pool = establish_connection()
            .await
            .expect("Failed to connect to the database");    

    let schema = schema::build_schema(pool.clone());            

    let static_files_service = ServeDir::new("assets");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::GET])
        .allow_headers([http::header::CONTENT_TYPE]);

    let app = Router::new()
        .nest_service("/assets", static_files_service)
        .route("/graphql", post(graphiql_handler))
        .route("/", get(root_handler))
        .route("/graphiql", get(graphiql_source))         
        .layer(cors)
        .with_state(schema);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct RustTemplate;

async fn root_handler() -> Html<String> {
    let template = RustTemplate;
    let html_content = template.render().unwrap();
    Html(html_content)
}