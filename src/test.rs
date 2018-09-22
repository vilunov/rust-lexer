use token::BinaryOperator::*;
use token::Token::*;
use token::*;

struct TestCase(&'static str, &'static [Token]);

const TESTS: &[TestCase] = &[
    TestCase(
        "255+1488",
        &[LiteralInt, BinaryOperator(Plus), LiteralInt, Eof],
    ),
    TestCase(
        "<>=<<=1",
        &[
            LessThan,
            GreaterEqual,
            BinaryOperatorAssignment(Shl),
            LiteralInt,
            Eof,
        ],
    ),
    TestCase(
        "\"This is a bucket!\"+\"Dear god\"",
        &[LiteralStr, BinaryOperator(Plus), LiteralStr, Eof],
    ),
    TestCase(
        "- = -====",
        &[
            BinaryOperator(Minus),
            Whitespace,
            Equal,
            Whitespace,
            BinaryOperatorAssignment(Minus),
            DoubleEqual,
            Equal,
            Eof,
        ],
    ),
    TestCase(
        "2+2//сложение чисел\n3+=3",
        &[
            LiteralInt,
            BinaryOperator(Plus),
            LiteralInt,
            Comment,
            Whitespace,
            LiteralInt,
            BinaryOperatorAssignment(Plus),
            LiteralInt,
            Eof,
        ],
    ),
];

#[test]
fn t1() {
    for TestCase(input, output) in TESTS {
        let tokens = tokenize(input.chars());
        assert_eq!(tokens, *output);
        println!("Expression {} tokenized successfully!", input);
    }
}

//#[test] //TODO uncomment when complete
fn _test_self() {
    use std::fs::{read_dir, read_to_string};
    for entry in read_dir("src")
        .unwrap()
        .map(|i| i.unwrap().path())
        .filter(|i| i.is_file())
        .filter(|i| i.extension().and_then(|i| i.to_str()) == Some("rs"))
    {
        println!("Tokenizing file {:?}", entry);
        let contents = read_to_string(entry).unwrap();
        let tokens = tokenize(contents.chars());
        println!("Tokenized into: {:#?}", tokens);
    }
}
