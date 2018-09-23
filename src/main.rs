use std::fs::{read_to_string, write};

pub mod token;

use token::Tokenizer;

const INPUT: &'static str = "in.txt";
const OUTPUT: &'static str = "out.txt";

fn main() {
    let test = read_to_string(INPUT).expect("something went wrong reading the file");
    let tokens = Tokenizer::new(test.chars());
    let output = tokens
        .map(|i| format!("{:?}", i))
        .collect::<Vec<_>>()
        .join("\n");
    write(OUTPUT, output).unwrap()
}

#[cfg(test)]
mod test;
