/// Token which is usually paired with another token, i.e. is either left or right
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PairedToken {
    /// `(` or `)`
    Parenthesis,
    /// `[` or `]`
    Bracket,
    /// `{` or `}`
    Brace,
}

/// Operator on two expressions returning an expression of the same value
///
/// Could be used with an assignment symbol when mutating a variable.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BinaryOperator {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `&`
    And,
    /// `|`
    Or,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
}

/// Token - a lexical unit of the program source code
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Token {
    // Delimeters
    /// Left (opening) part of paired token, e.g. `(`
    Left(PairedToken),
    /// Right (closing) part of paired token, e.g. `)`
    Right(PairedToken),
    /// Delimiting whitespace
    Whitespace,
    /// `=`
    Equal,
    /// `==`
    DoubleEqual,
    /// `<`
    LessThan,
    /// `<=`
    LessEqual,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterEqual,
    /// `!=`
    NotEqual,
    /// `&&`
    DoubleAnd,
    /// `||`
    DoubleOr,
    /// `!`
    Not,
    /// `~`
    Tilde,

    // Operators
    /// Binary operator, e.g. `+`
    BinaryOperator(BinaryOperator),
    /// Binary operator with assignment, e.g. `+=`
    BinaryOperatorAssignment(BinaryOperator),

    // Literals
    LiteralInt,
    LiteralStr,

    Identifier,

    /// End of stream
    Eof,
}

fn char_to_binop(c: char) -> Option<BinaryOperator> {
    match c {
        '+' => Some(BinaryOperator::Plus),
        '-' => Some(BinaryOperator::Minus),
        '*' => Some(BinaryOperator::Star),
        '/' => Some(BinaryOperator::Slash),
        '%' => Some(BinaryOperator::Percent),
        '^' => Some(BinaryOperator::Caret),
        '&' => Some(BinaryOperator::And),
        '|' => Some(BinaryOperator::Or),
        _ => None,
    }
}

/// Tokenize the stream of incoming source code
///
/// # Panics
///
/// When the tokenizer encounters an unexpected character
pub fn tokenize<S>(stream: S) -> Vec<Token>
where
    S: Iterator<Item = char>,
{
    use self::BinaryOperator::*;
    use self::Token::*;

    let mut stream = stream.peekable();
    let mut output = vec![];

    while let Some(&c) = stream.peek() {
        // === Binary operators ===
        if let Some(binop) = char_to_binop(c) {
            stream.next();
            match stream.peek() {
                Some('=') => {
                    output.push(BinaryOperatorAssignment(binop));
                    stream.next();
                }
                _ => {
                    output.push(BinaryOperator(binop));
                }
            }
            continue;
        }
        // === Numerical literals ===
        if c.is_ascii_digit() {
            output.push(LiteralInt);
            while let Some(_) = stream.peek().filter(|i| i.is_ascii_digit()) {
                stream.next();
            }
            continue;
        }
        match c {
            // === String literals ===
            // TODO character escaping and other types of literals
            '\"' => {
                output.push(LiteralStr);
                stream.next();
                while stream.peek() != Some(&'\"') {
                    stream.next();
                }
                stream.next();
            }
            // === Comparison operators ===
            '<' => {
                stream.next();
                if let Some(&second_char) = stream.peek() {
                    match second_char {
                        '=' => {
                            output.push(LessEqual);
                            stream.next();
                        }
                        '<' => {
                            stream.next();
                            if stream.peek() == Some(&'=') {
                                output.push(BinaryOperatorAssignment(Shl));
                                stream.next();
                            } else {
                                output.push(BinaryOperator(Shl));
                            }
                        }
                        _ => {
                            output.push(LessThan);
                        }
                    }
                } else {
                    output.push(LessThan);
                }
            }
            '>' => {
                stream.next();
                if let Some(&second_char) = stream.peek() {
                    match second_char {
                        '=' => {
                            output.push(GreaterEqual);
                            stream.next();
                        }
                        '>' => {
                            stream.next();
                            if stream.peek() == Some(&'=') {
                                output.push(BinaryOperatorAssignment(Shr));
                                stream.next();
                            } else {
                                output.push(BinaryOperator(Shr));
                            }
                        }
                        _ => {
                            output.push(GreaterThan);
                        }
                    }
                } else {
                    output.push(GreaterThan);
                }
            }
            '=' => {
                stream.next();
                match stream.peek() {
                    Some('=') => {
                        output.push(DoubleEqual);
                        stream.next();
                    }
                    _ => {
                        output.push(Equal);
                    }
                }
            }
            ' ' | '\n' => {
                output.push(Whitespace);
                stream.next();
            }

            _ => panic!("Unexpected character {}", c),
        }
    }
    output.push(Eof);

    output
}
