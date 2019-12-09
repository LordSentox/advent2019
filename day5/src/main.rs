#[macro_use]
extern crate enum_primitive_derive;

mod int_machine;

use int_machine::{self as machine, Int};
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Could not read input file");

    // Convert the input to a `Vec`.
    let tape: Vec<Int> = input
        .split(',')
        .map(|x| {
            x.trim()
                .parse::<Int>()
                .expect("Unable to parse input string. Not all numbers")
        })
        .collect();

    machine::run(tape);
}
