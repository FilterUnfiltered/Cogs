fn main() {
    cogs::build(std::env::current_dir().unwrap().join("cogs")).unwrap();
}
