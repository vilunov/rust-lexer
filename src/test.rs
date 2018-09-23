use token::BinaryOperator::*;
use token::PairedToken::*;
use token::Token::*;
use token::*;

struct TestCase(&'static str, &'static [Token]);

const TESTS: &[TestCase] = &[
    TestCase("255+1488", &[LiteralInt, BinaryOperator(Plus), LiteralInt]),
    TestCase(
        "<>=<<=1",
        &[
            LessThan,
            GreaterEqual,
            BinaryOperatorAssignment(Shl),
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
        "  let mut a = 3;",
        &[
            Whitespace,
            Identifier,
            Whitespace,
            Identifier,
            Whitespace,
            Identifier,
            Whitespace,
            Equal,
            Whitespace,
            LiteralInt,
            Semicolon
        ]
    ),
    TestCase(
        "let mut vec = Vec::new(   );",
        &[
            Identifier,
            Whitespace,
            Identifier,
            Whitespace,
            Identifier,
            Whitespace,
            Equal,
            Whitespace,
            Identifier,
            DoubleColon,
            Identifier,
            Left(Parenthesis),
            Whitespace,
            Right(Parenthesis),
            Semicolon
        ]
    ),
    TestCase(
                "let vec = vec![0; 5];\n
                while let Some(top) = stack.pop() {\n// Prints 3, 2, 1\n
                println!(\"{}\", top);\n
                }"
        ,
        &[
            Identifier,
            Whitespace,
            Identifier,
            Whitespace,
            Equal,
            Whitespace,
            Identifier,
            Exclamation,
            Left(Bracket),
            LiteralInt,
            Semicolon,
            Whitespace,
            LiteralInt,
            Right(Bracket),
            Semicolon,
            Whitespace,

            Identifier,
            Whitespace,
            Identifier,
            Whitespace,
            Identifier,
            Left(Parenthesis),
            Identifier,
            Right(Parenthesis),
            Whitespace,
            Equal,
            Whitespace,
            Identifier,
            Dot,
            Identifier,
            Left(Parenthesis),
            Right(Parenthesis),
            Whitespace,
            Left(Brace),
            Whitespace,
            Comment,
            Whitespace,

            Identifier,
            Exclamation,
            Left(Parenthesis),
            LiteralStr,
            Comma,
            Whitespace,
            Identifier,
            Right(Parenthesis),
            Semicolon,
            Whitespace,

            Right(Brace)
        ]
    ),
    TestCase(
        "2-+6*7^311231;\n",
        &[
            LiteralInt,
            BinaryOperator(Minus),
            BinaryOperator(Plus),
            LiteralInt,
            BinaryOperator(Star),
            LiteralInt,
            BinaryOperator(Caret),
            LiteralInt,
            Semicolon,
            Whitespace
        ]
    ), TestCase(
        "a<<=(2|643);\n
        b>>=(234242&(2424234%0))",
        &[
            Identifier,
            BinaryOperatorAssignment(Shl),
            Left(Parenthesis),
            LiteralInt,
            BinaryOperator(Or),
            LiteralInt,
            Right(Parenthesis),
            Semicolon,
            Whitespace,

            Identifier,
            BinaryOperatorAssignment(Shr),
            Left(Parenthesis),
            LiteralInt,
            BinaryOperator(And),
            Left(Parenthesis),
            LiteralInt,
            BinaryOperator(Percent),
            LiteralInt,
            Right(Parenthesis),
            Right(Parenthesis)
        ]
    )

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
