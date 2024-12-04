use axum::{response::Html, routing::get, Router};

cogs_runtime::cogs_mod!(index);
use cogs_runtime::Component;

async fn index() -> Html<String> {
    Html(index::Cog.render(()).await.unwrap())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
