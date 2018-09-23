use token::BinaryOperator::*;
use token::PairedToken::*;
use token::Token::*;
use token::*;

struct TestCase(&'static str, &'static [Token]);

const TESTS: &[TestCase] = &[
    TestCase("255+1488", &[LiteralInt, BinaryOperator(Plus), LiteralInt]),
    TestCase(
        "<>=<<=1 != ==5",
        &[
            LessThan,
            GreaterEqual,
            BinaryOperatorAssignment(Shl),
            LiteralInt,
            Whitespace,
            NotEqual,
            Whitespace,
            DoubleEqual,
            LiteralInt,
        ],
    ),
    TestCase(
        "\"This is a bucket!\"+\"Dear\\ngod\"",
        &[LiteralStr, BinaryOperator(Plus), LiteralStr],
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
        ],
    ),
    TestCase(
        "2+/* block comment */3",
        &[LiteralInt, BinaryOperator(Plus), Comment, LiteralInt],
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
        ],
    ),
    TestCase(
        "fn kek<'cool_lifetyme_1337, T>(shrek: & 'cool_lifetyme_1337 mut T) -> T {}",
        &[
            Identifier,
            Whitespace,
            Identifier,
            LessThan,
            IdentifierLifetime,
            Comma,
            Whitespace,
            Identifier,
            GreaterThan,
            Left(Parenthesis),
            Identifier,
            Colon,
            Whitespace,
            BinaryOperator(And),
            Whitespace,
            IdentifierLifetime,
            Whitespace,
            Identifier,
            Whitespace,
            Identifier,
            Right(Parenthesis),
            Whitespace,
            RightArrow,
            Whitespace,
            Identifier,
            Whitespace,
            Left(Brace),
            Right(Brace),
        ],
    ),
];

fn tokenize(input: &str) -> Vec<Token> {
    Tokenizer::new(input.chars()).collect()
}

#[test]
fn test_custom_positive() {
    for TestCase(input, output) in TESTS {
        let tokens = tokenize(&input);
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
        let tokens = tokenize(&contents);
        println!("Tokenized into: {:#?}", tokens);
    }
}

#[test]
fn test_self() {
    test_on_folder("src");
}

#[test]
fn test_rustc() {
    test_on_folder("test");
}
