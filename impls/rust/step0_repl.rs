use std::io::{self, BufRead, Write};

fn main() {
    loop {
        print!("user> ");
        io::stdout().flush();
        let input = io::stdin().lock().lines().next().unwrap().unwrap();
        println!("{}", rep(input));
    }
}

fn read(val: String) -> String { val }
fn eval(val: String) -> String { val }
fn print(val: String) -> String { val }
fn rep(val: String) -> String {
    print(eval(read(val)))
}