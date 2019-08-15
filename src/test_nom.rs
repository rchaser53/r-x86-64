#![feature(exclusive_range_pattern)]

extern crate nom;

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
pub enum Statement {
    Bin(Expr, Atom, Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(i32),
    Keyword(String),
    Boolean(bool),
    BuiltIn(BuiltIn),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltIn {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
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

fn parse_builtin_op<'a>(i: &'a str) -> IResult<&'a str, BuiltIn, VerboseError<&'a str>> {
    let (i, t) = one_of("+-*/=")(i)?;

    Ok((
        i,
        match t {
            '+' => BuiltIn::Plus,
            '-' => BuiltIn::Minus,
            '*' => BuiltIn::Times,
            '/' => BuiltIn::Divide,
            '=' => BuiltIn::Equal,
            _ => unreachable!(),
        },
    ))
}

fn parse_bool<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
        map(tag("true"), |_| Atom::Boolean(true)),
        map(tag("false"), |_| Atom::Boolean(false)),
    ))(i)
}

fn parse_atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((parse_num, parse_bool))(i)
}

fn parse_constant<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    map(parse_atom, |atom| Expr::Constant(atom))(i)
}

fn parse_expr<'a>(i: &'a str) -> IResult<&'a str, Statement, VerboseError<&'a str>> {
    let (input, (left, op, right)) = tuple((
        preceded(multispace0, parse_constant),
        preceded(multispace0, map(parse_builtin_op, Atom::BuiltIn)),
        preceded(multispace0, parse_constant),
    ))(i)?;
    Ok((input, Statement::Bin(left, op, right)))
}

fn play_nom() {
    let mut input = "111 + 234";
    while input != "" {
        let (left, result) = parse_expr(input).unwrap();
        dbg!(result);
        input = left;
    }
}
