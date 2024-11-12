use cogs_parser::parse_cog;

fn main() {
    println!("{:#?}", parse_cog("{println!(\"Hello\")}\n<h1>hi</h1>"));
}