use token::BinaryOperator::*;
use token::PairedToken::*;
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
        "\"This is a bucket!\"+\"Dear\ngod\"",
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
    TestCase(
        "2+/* block comment */3",
        &[LiteralInt, BinaryOperator(Plus), Comment, LiteralInt, Eof],
    ),
    TestCase(
        "struct TestCase(&'static str, &'static [Token]);",
        &[
            Identifier,
            Whitespace,
            Identifier,
            Left(Parenthesis),
            BinaryOperator(And),
            IdentifierLifetime,
            Whitespace,
            Identifier,
            Comma,
            Whitespace,
            BinaryOperator(And),
            IdentifierLifetime,
            Whitespace,
            Left(Bracket),
            Identifier,
            Right(Bracket),
            Right(Parenthesis),
            Semicolon,
            Eof,
        ],
    ),
];

#[test]
fn test_custom_positive() {
    for TestCase(input, output) in TESTS {
        let tokens = tokenize(input.chars());
        assert_eq!(tokens, *output);
        println!("Expression {} tokenized successfully!", input);
    }
}

fn test_on_folder(folder_name: &str) {
    use std::fs::{read_dir, read_to_string};

    for entry in read_dir(folder_name)
        .unwrap()
        .map(|i| i.unwrap().path())
        .filter(|i| i.is_file())
        .filter(|i| i.extension().and_then(|i| i.to_str()) == Some("rs"))
    {
        println!("Tokenizing file {:?}", entry);
        let contents = read_to_string(entry).unwrap();
        let tokens = tokenize(contents.chars());
        assert_eq!(tokens.last(), Some(&Eof));
        println!("Tokenized into: {:#?}", tokens);
    }
}

#[test]
fn test_self() {
    test_on_folder("src");
}
