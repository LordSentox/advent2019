use num_traits::FromPrimitive;
use std::io::{stdin, stdout, Write};

// The integer type used in the inputs
pub type Int = i64;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Primitive)]
pub enum Operation {
    Add = 1,
    Mul = 2,
    Inp = 3,
    Out = 4,
    Exit = 99,
}

impl Operation {
    pub fn num_params(self) -> usize {
        use Operation::*;
        match self {
            Add => 3,
            Mul => 3,
            Inp => 1,
            Out => 1,
            Exit => 0,
        }
    }
}

pub enum Mode {
    Address,
    Immediate,
}

fn split_op_code(op_code: Int) -> (Operation, Vec<Mode>) {
    let op = Operation::from_u8((op_code % 100) as u8).expect("Invalid op code");

    // Firstly, we will ignore the operation part of our opcode, so the least significant two
    // digits will be cut off.
    let mut op_code = op_code / 100;
    let mut modes = Vec::new();
    while op_code != 0 {
        match op_code % 10 {
            1 => modes.push(Mode::Immediate),
            0 => modes.push(Mode::Address),
            other => panic!("{} is not a valid parameter mode", other),
        }

        op_code /= 10;
    }

    (op, modes)
}

fn resolve_pos(pos: usize, modes: &Vec<Mode>, tape: &Vec<Int>) -> usize {
    match modes.get(pos) {
        Some(Mode::Address) | None => tape[pos] as usize,
        Some(Mode::Immediate) => pos,
    }
}

fn add(tape: &mut Vec<Int>, pos: usize, modes: Vec<Mode>) {
    let res_pos = tape[pos + 3] as usize;
    let p1_pos = resolve_pos(pos + 1, &modes, &tape);
    let p2_pos = resolve_pos(pos + 2, &modes, &tape);

    print!(
        "@{}[{}] + @{}[{}] => @{}[{:?} ->",
        p1_pos, tape[p1_pos], p2_pos, tape[p2_pos], res_pos, tape[res_pos]
    );

    tape[res_pos] = tape[p1_pos] + tape[p2_pos];

    println!(" {}]", tape[res_pos]);
}

fn mul(tape: &mut Vec<Int>, pos: usize, modes: Vec<Mode>) {
    let res_pos = tape[pos + 3] as usize;
    let p1_pos = resolve_pos(pos + 1, &modes, &tape);
    let p2_pos = resolve_pos(pos + 2, &modes, &tape);

    print!(
        "@{}[{}] * @{}[{}] => @{}[{:?} ->",
        p1_pos, tape[p1_pos], p2_pos, tape[p2_pos], res_pos, tape[res_pos]
    );

    tape[res_pos] = tape[p1_pos] * tape[p2_pos];

    println!(" {}]", tape[res_pos]);
}

fn inp(tape: &mut Vec<Int>, pos: usize) {
    let mut input = String::new();
    print!("Please enter input num: ");

    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Could not read string from input");
    let input = input
        .trim()
        .parse::<Int>()
        .expect("Did not enter correct integer");

    let input_pos = resolve_pos(pos + 1, &vec![Mode::Address], &tape);
    tape[input_pos] = input;
    println!("Storing input at {}", input_pos);
}

fn out(tape: &Vec<Int>, pos: usize, modes: Vec<Mode>) {
    let out_pos = resolve_pos(pos + 1, &modes, &tape);

    println!("Output: {}", tape[out_pos]);
}

// Run the instructions on the tape provided. Returns the tape after the calculation is done
pub fn run(mut tape: Vec<Int>) -> Vec<Int> {
    let mut pos: usize = 0;
    loop {
        let (op, modes) = split_op_code(tape[pos]);

        match op {
            Operation::Add => add(&mut tape, pos, modes),
            Operation::Mul => mul(&mut tape, pos, modes),
            Operation::Inp => inp(&mut tape, pos),
            Operation::Out => out(&tape, pos, modes),
            Operation::Exit => break tape,
        }

        // Go to the next instruction
        pos += op.num_params() + 1;
    }
}
