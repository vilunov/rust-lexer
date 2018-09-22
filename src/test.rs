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
];

#[test]
fn t1() {
    for TestCase(input, output) in TESTS {
        let tokens = tokenize(input.chars());
        assert_eq!(tokens, *output);
        println!("Expression {} tokenized successfully!", input);
    }
}
