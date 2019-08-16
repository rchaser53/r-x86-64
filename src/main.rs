// pub mod assemble;
// use assemble::*;

#[macro_use] extern crate lalrpop_util;

pub mod calculator1;
use calculator1::TermParser;

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

fn main() {
    dbg!(calculator1::TermParser::new().parse("22"));
}
