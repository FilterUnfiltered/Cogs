use axum::{response::Html, routing::get, Router};

cogs_runtime::cogs_mod!(index);
use cogs_runtime::Component;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(cogs_axum::serve_cog::<index::Cog>));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
