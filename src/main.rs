#![feature(exclusive_range_pattern)]

use std::env;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません\n");
    }

    let _ = args.next();
    if let Some(second) = args.next() {
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("main:");
        parse(&second);
        println!("  ret");
    }
}

pub enum Op {
    Plus,
    Minus,
}

fn consume_first_value(chars: &mut Peekable<Chars>) {
    let mut current_value = String::new();
    while let Some(next_char) = chars.peek() {
        match next_char {
            value @ '0'..'9' => {
                if current_value == "0" {
                    panic!("failed");
                }
                current_value.push(*value);
                chars.next();
            }
            _ => {
                break;
            }
        }
    }

    if current_value.is_empty() {
        panic!("failed");
    } else {
        println!("  mov rax, {}", &current_value.parse::<i32>().unwrap())
    }
}

fn calc_value(opcode: Option<Op>, current_value: &str) {
    if !current_value.is_empty() {
        emit_assemble_str(opcode, &current_value);
    } else {
        if let Some(_) = opcode {
            panic!("failed");
        }
    }
}

fn parse(input: &str) {
    let mut chars = input.chars().peekable();
    let mut opcode = None;
    let mut current_value = String::new();
    consume_first_value(&mut chars);
    while let Some(next_char) = chars.next() {
        match next_char {
            '+' => {
                calc_value(opcode, &current_value);
                opcode = Some(Op::Plus);
                current_value = String::new();
            }
            '-' => {
                calc_value(opcode, &current_value);
                opcode = Some(Op::Minus);
                current_value = String::new();
            }
            value @ '0'..'9' => {
                if current_value == "0" {
                    panic!("failed");
                }
                current_value.push(value);
            }
            _ => {
                panic!("failed");
            }
        }
    }

    emit_assemble_str(opcode, &current_value);
}

fn emit_assemble_str(opcode: Option<Op>, current_value: &str) {
    match opcode {
        Some(Op::Plus) => println!("  add rax, {}", current_value.parse::<i32>().unwrap()),
        Some(Op::Minus) => println!("  sub rax, {}", current_value.parse::<i32>().unwrap()),
        _ => {}
    }
}
