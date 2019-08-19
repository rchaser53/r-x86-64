// pub mod assemble;
// use assemble::*;

#[macro_use] extern crate lalrpop_util;

pub mod calculator;
use calculator::ExprsParser;

pub mod ast;

fn main() {
    let mut errors = Vec::new();
    dbg!(ExprsParser::new().parse(&mut errors, "12 + 34 * 56"));
}
