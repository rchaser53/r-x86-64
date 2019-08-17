// pub mod assemble;
// use assemble::*;

#[macro_use] extern crate lalrpop_util;

pub mod calculator;
// use calculator::TermParser;

pub mod calculator2;
// use calculator2::TermParser;

#[test]
fn calculator() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());
}

fn main() {
    dbg!(calculator::TermParser::new().parse("22"));
    dbg!(calculator2::TermParser::new().parse("22"));
}
