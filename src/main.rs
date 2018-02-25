// Really simple script that randomizes a bitwise operator
// and operands, and calculates result.
// Things I'd like to do:
//   * Improve formatting, so that printed values are leftpadded
//   * Wait for user input, and check correctness of answer
//   * Come up with suitable limit for Shift operands

extern crate rand;
#[macro_use]
extern crate rand_derive;
use rand::Rng;

#[derive(Debug, Rand)]
enum Operator {
    And,
    Or,
    XOr,
    // LShift,
    // RShift,
}

fn execute(values: (u8, u8), operator: Operator) -> u16 {
    match operator {
        Operator::And    => ( values.0 & values.1 ) as u16,
        Operator::Or     => ( values.0 | values.1 ) as u16,
        Operator::XOr    => ( values.0 ^ values.1 ) as u16,
        // Must limit operand size before attempting to use these
        // Operator::LShift => ( values.0 << values.1 ) as u16,
        // Operator::RShift => ( values.0 >> values.1 ) as u16,
    }
}

fn random_operator() -> Operator {
    let mut rng = rand::thread_rng();
    let operator: Operator = rng.gen();
    operator
}

fn main() {
    let a = rand::random::<u8>();
    println!("{:b}", a);
    let operator = random_operator();
    println!("{:?}", operator);
    let b = rand::random::<u8>();
    println!("{:b}", b);
    let c = execute((a, b), operator);
    println!("{:b}", c);
  }
