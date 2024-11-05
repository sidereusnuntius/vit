fn main() {
    // lalrpop::Configuration::new()..process_file(path)..process_dir("src/parser").unwrap();
    lalrpop::process_root().unwrap();
}