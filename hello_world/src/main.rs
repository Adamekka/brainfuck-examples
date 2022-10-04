use std::fs;
use brainfuck;

fn main() {
    let brainfuck_code: String = fs::read_to_string("./brainfuck.bf")
        .expect("Error: brainfuck.bf missing");

    brainfuck::eval_string(&brainfuck_code)
        .expect("Something went wrong");
}
