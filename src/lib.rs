mod diagnostics;
#[cfg(test)]
mod tests;

use std::path::Path;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;

#[doc(hidden)]
pub fn parse_cog(input: String, file: &str) -> eyre::Result<cogs_ast::Component> {
    use cogs_parser::nom::Finish;

    match cogs_parser::parse_cog(&input).finish() {
        Ok((leftover, ast)) => {
            if leftover.is_empty() {
                Ok(ast)
            } else {
                Err(eyre::eyre!("Not all input parsed, leftover: {leftover}; parsed ast: {ast:#?}"))
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
    let registry = tracing_subscriber::registry().with(
        tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );
    fn do_init<
        R: tracing::Subscriber
            + SubscriberExt
            + for<'span> tracing_subscriber::registry::LookupSpan<'span>
            + Into<tracing::Dispatch>
            + Sync
            + Send
            + 'static,
    >(
        registry: R,
    ) -> eyre::Result<()> {
        tracing::subscriber::set_global_default(
            registry
                .with(tracing_error::ErrorLayer::default())
                .with(tracing_subscriber::fmt::layer()),
        )
        .map_err(Into::into)
    }
    #[cfg(feature = "tracy")]
    {
        do_init(registry.with(tracing_tracy::TracyLayer::default()))?;
    }
    #[cfg(not(feature = "tracy"))]
    {
        do_init(registry)?;
    }

    Ok(())
}

pub fn build(dir: impl AsRef<Path>) -> eyre::Result<()> {
    let dir = dir.as_ref();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            build(&path)?;
        } else if path.extension().map_or(false, |ext| ext == "cog") {
            let _span = tracing::debug_span!("build cog", path = %path.display());
            let contents = std::fs::read_to_string(&path)?;
            let readable_path = if let Some(diffed) = std::env::current_dir()
                .ok()
                .and_then(|cwd| pathdiff::diff_paths(&path, &cwd))
            {
                diffed.display().to_string()
            } else {
                path.display().to_string()
            };
            let ast = parse_cog(contents, &readable_path)?;
            tracing::debug!(?ast, "parsed");
            let code = cogs_codegen::generate(&ast)?;
            tracing::trace!(?code, "generated");
            std::fs::write(
                Path::new(&out_dir).join(
                    pathdiff::diff_paths(&path, dir)
                        .expect("path is not relative to dir for some reason")
                        .with_extension("rs"),
                ),
                code,
            )?;
        }
    }

    Ok(())
}
