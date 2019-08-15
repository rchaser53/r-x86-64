extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
    // lalrpop::Configuration::new()
        // .emit_comments(true)
        // .force_build(true)
        // .unit_test()
        // .log_debug()
        // .set_in_dir(".")
        // .set_out_dir(".");
        // .unwrap();
}