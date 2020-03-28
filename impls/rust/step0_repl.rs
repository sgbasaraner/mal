extern crate linefeed;

use linefeed::{Interface, ReadResult};

fn main() {
    let reader = Interface::new("rust-mal").expect("Couldn't initialize reader.");

    reader.set_prompt("user> ").expect("Couldn't set reader prompt.");

    while let ReadResult::Input(input) = reader.read_line().expect("Couldn't read line.") {
        println!("{}", rep(input.clone()));
        if !input.trim().is_empty() {
            reader.add_history(input);
        }
    }
}

fn read(val: String) -> String { val }
fn eval(val: String) -> String { val }
fn print(val: String) -> String { val }
fn rep(val: String) -> String {
    print(eval(read(val)))
}