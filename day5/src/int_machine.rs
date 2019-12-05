use std::fs;
use std::io::{stdin, stdout, Write};

// The integer type used in the inputs
pub type Int = u32;

// The exit code used by the int input programs
const CODE_EXIT: Int = 99;

const CODE_ADD: Int = 1;
const CODE_MUL: Int = 2;
const CODE_INP: Int = 3;
const CODE_OUT: Int = 4;

pub enum Mode {
    Address,
    Immediate
}

pub fn modes_from_opcode()

fn add(tape: &mut Vec<Int>, pos: usize, modes: Vec<Mode>) {
    let res_pos = tape[pos + 3] as usize;

    print!(
        "@{}[{}] + @{}[{}] => @{}[{:?} ->",
        tape[pos + 1],
        tape[tape[pos + 1] as usize],
        tape[pos + 2],
        tape[tape[pos + 2] as usize],
        tape[pos + 3],
        tape[res_pos]
    );

    tape[res_pos] = tape[tape[pos + 1] as usize] + tape[tape[pos + 2] as usize];

    println!(" {}]", tape[res_pos]);
}

fn mul(tape: &mut Vec<Int>, pos: usize) {
    let res_pos = tape[pos + 3] as usize;

    print!(
        "@{}[{}] * @{}[{}] => @{}[{:?} ->",
        tape[pos + 1],
        tape[tape[pos + 1] as usize],
        tape[pos + 2],
        tape[tape[pos + 2] as usize],
        tape[pos + 3],
        tape[res_pos]
    );

    tape[res_pos] = tape[tape[pos + 1] as usize] * tape[tape[pos + 2] as usize];

    println!(" {}]", tape[res_pos]);
}

fn inp(tape: &mut Vec<Int>, pos: usize) {
    let mut input = String::new();
    print!("Please enter input num: ");

    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Could not read string from input");
    input.trim();

    let input = input.parse::<Int>().expect("Did not enter correct integer");
}

// Run the instructions on the tape provided. Returns the tape after the calculation is done
pub fn run(mut tape: Vec<Int>) -> Vec<Int> {
    let mut pos: usize = 0;
    while tape[pos] != CODE_EXIT {
        match tape[pos] {
            CODE_ADD => add(&mut tape, pos),
            CODE_MUL => mul(&mut tape, pos),
            other => panic!("Found invalid opcode [{}]", other),
        }

        // Go to the next instruction
        pos += 4;
    }

    tape
}

fn main() {
    let input = fs::read_to_string("input").expect("Could not read input file");

    // Convert the input to a `Vec`.
    let mut tape: Vec<Int> = input
        .split(',')
        .map(|x| {
            x.trim()
                .parse::<Int>()
                .expect("Unable to parse input string. Not all numbers")
        })
        .collect();

    // Set to the 1202 error state
    tape[1] = 12;
    tape[2] = 2;

    println!("Running part one");

    let solution = run(tape.clone())[0];
    println!("The output is {}", solution);

    println!("Running part two");

    // Run through all number combinations. Since add and mul are commutative, j can start at i (i
    // <= j)
    for i in 0..100 {
        for j in i..100 {
            println!("Running tape with inputs ({}, {})", i, j);

            tape[1] = i;
            tape[2] = j;

            let result = run(tape.clone())[0];
            println!("Result is: {}", result);
            if result == PART_TWO_EXPECTED {
                println!("--------------------------------------------------------------");
                println!(
                    "The smallest solution for part two was found: {}",
                    i * 100 + j
                );
                println!("--------------------------------------------------------------");
                break;
            }
        }
    }
}
