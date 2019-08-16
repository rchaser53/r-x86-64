extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .set_in_dir("src")
        .generate_in_source_tree()
        .process()
        .unwrap()
}