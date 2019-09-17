// pub mod assemble;
// use assemble::*;

#[macro_use]
extern crate lalrpop_util;

pub mod calculator;
use calculator::ExprsParser;
use calculator::StatementsParser;
pub mod ast;

fn main() {
    let mut errors = Vec::new();
    dbg!(ExprsParser::new().parse(&mut errors, "12 + 34 * 56"));
    dbg!(ExprsParser::new().parse(&mut errors, "AbcdE"));
    dbg!(StatementsParser::new().parse(&mut errors, "let 22 = 123;"));
}
