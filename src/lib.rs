mod diagnostics;
#[cfg(test)]
mod tests;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[doc(hidden)]
pub fn parse_cog(input: String, file: &str) -> eyre::Result<cogs_ast::Component> {
    use cogs_parser::nom::Finish;

    match cogs_parser::parse_cog(&input).finish() {
        Ok((leftover, ast)) => {
            if leftover.is_empty() {
                Ok(ast)
            } else {
                Err(eyre::eyre!("Not all input parsed, leftover: {leftover}"))
            }
        }
        Err(error) => {
            diagnostics::nom_diagnostic(&input, error, file);
            Err(eyre::Report::msg("parsing failed"))
        }
    }
}

#[doc(hidden)]
pub fn init_tracing() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_error::ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;
    Ok(())
}
