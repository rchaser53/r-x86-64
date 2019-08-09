use std::env;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません\n");
    }

    let _ = args.next();
    if let Some(second) = args.next() {
        println!(".intel_syntax noprefix\n");
        println!(".global main\n");
        println!("main:\n");
        println!("  mov rax, {}\n", second.parse::<i32>().unwrap());
        println!("  ret\n");
    }
}
