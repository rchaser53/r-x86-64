use std::env;

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
        println!("  mov rax, {}", second.parse::<i32>().unwrap());
        println!("  ret");
    }
}
