use axum::{http::StatusCode, response::Html};

/// Serve a .cog file
///
/// This function is the handler to serve a .cog file.
/// It takes the type of the generated component as a type argument.
///
/// # Example
/// ```ignore
/// cogs_runtime::cogs_mod!(index); // index.cog
///
/// let app = Router::new().route("/", get(cogs_axum::serve_cog::<index::Cog>)); // note the
/// turbofish here
/// ```
pub async fn serve_cog<C: cogs_runtime::Component + Default>(
) -> Result<Html<String>, (StatusCode, String)>
where
    C::Error: std::fmt::Display + Send + Sync + 'static,
    C::Props: Default,
{
    let html = C::default()
        .render(Default::default())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(html))
}
