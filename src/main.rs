#![feature(exclusive_range_pattern)]

extern crate nom;

// pub mod assemble;
// use assemble::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(i32),
    Keyword(String),
    Boolean(bool),
}

fn parse_num<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
        map_res(digit1, |digit_str: &str| {
            digit_str.parse::<i32>().map(Atom::Num)
        }),
        map(preceded(tag("-"), digit1), |digit_str: &str| {
            Atom::Num(-1 * digit_str.parse::<i32>().unwrap())
        }),
    ))(i)
}

fn parse_atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    parse_num(i)
}

fn parse_constant<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    map(parse_atom, |atom| Expr::Constant(atom))(i)
}

fn parse_expr<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    preceded(multispace0, parse_constant)(i)
}

fn main() {
    let abc = parse_expr("123");
    dbg!(abc);
}
