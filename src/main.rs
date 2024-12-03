use cogs::*;

fn main() -> eyre::Result<()> {
    init_tracing()?;
    let ast = parse_cog(include_str!("../tests/1.cog").to_owned(), "tests/1.cog")?;
    println!("{ast:#?}");

    Ok(())
}
