// Really simple script that randomizes a bitwise operator
// and operands, and calculates result.
// Things I'd like to do:
//   * Improve formatting, so that printed values are leftpadded
//   * Check correctness of answer
//   * Come up with suitable limit for Shift operands

use std::io;

extern crate rand;
#[macro_use]
extern crate rand_derive;
use rand::Rng;
use rand::distributions::{Range, Sample};

#[derive(Debug, Rand, PartialEq)]
enum Operator {
    And,
    Or,
    XOr,
    LShift,
    RShift,
}

fn execute(values: (u32, u32), operator: Operator) -> u32 {
    match operator {
        Operator::And    => ( values.0 & values.1 ),
        Operator::Or     => ( values.0 | values.1 ),
        Operator::XOr    => ( values.0 ^ values.1 ),
        Operator::LShift => ( values.0 << values.1 ),
        Operator::RShift => ( values.0 >> values.1 ),
    }
}

fn random_operator() -> Operator {
    // Could maybe be static variable used throughout the code
    let mut rng = rand::thread_rng();
    let operator: Operator = rng.gen();
    operator
}

// To implement default parameter values in Rust, use Option
fn random_operands(ranges: Option<(Range<u32>, Range<u32>)>) -> (u32, u32) {
    match ranges {
        None => {
            (rand::random::<u8>() as u32, rand::random::<u8>() as u32)
        },
        Some(ranges) => {
            // ranges must be mutable to be sampled
            // cloning is simpler than revising whole mutability chain
            let mut _ranges = ranges.clone();
            let mut rng = rand::thread_rng();
            let a = _ranges.0.sample(&mut rng);
            let b = _ranges.1.sample(&mut rng);
            (a, b)
        },
    }
}

fn main() {
    let operator = random_operator();

    let ranges = match operator {
        // Operand values are capped to 1) stay within variable size limits
        // and 2) to produce small but varied enough combinations, for manual
        // calculation
        Operator::LShift => Some((Range::new(1, 127), Range::new(1, 7))),
        Operator::RShift => Some((Range::new(127, 255), Range::new(1,7))),
        _ => None
    };

    let values = random_operands(ranges);
    println!("What is the result of the following operation?");
    print!("  {:b}", values.0);
    print!(" {:?} ", operator);
    println!("{:b}", values.1);

    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line");

    let correct_answer = execute(values, operator);
    println!("Your answer: {:?}", answer.trim());
    println!("Correct answer: {:b}", correct_answer);
}
