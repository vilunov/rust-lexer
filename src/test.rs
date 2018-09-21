use token::tokenize;
use token::BinaryOperator::*;
use token::Token::*;

#[test]
fn t1() {
    let input = "255+255";
    let tokens = tokenize(input.chars());
    assert_eq!(
        tokens,
        vec![LiteralInt, BinaryOperator(Plus), LiteralInt, Eof]
    );
}
