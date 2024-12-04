fn main() {
    let _ = cogs::init_tracing();
    cogs::build(std::env::current_dir().unwrap().join("cogs")).unwrap();
}
