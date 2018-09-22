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
    /// Comments (including docs)
    Comment,
    /// `=`
    Equal,

    // Language structural tokens
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `::`
    DoubleColon,
    /// `;`
    Semicolon,
    /// `!`
    Exclamation,

    // Operators
    /// Binary operator, e.g. `+`
    BinaryOperator(BinaryOperator),
    /// Binary operator with assignment, e.g. `+=`
    BinaryOperatorAssignment(BinaryOperator),
    // Unary operators
    /// `!`
    Not,
    /// `~`
    Tilde,
    // Boolean and comparison operators
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
    /// `#`
    Sharp,

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
        // '/' => Some(BinaryOperator::Slash), // Handled as a special case
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
    use self::PairedToken::*;
    use self::Token::*;

    let mut stream = stream.peekable();
    let mut output = vec![];

    macro_rules! consume {
        ($token: expr) => {{
            stream.next();
            output.push($token);
        }};
    }

    while let Some(&c) = stream.peek() {
        // === Binary operators ===
        if let Some(binop) = char_to_binop(c) {
            stream.next();
            match stream.peek() {
                Some('=') => consume!(BinaryOperatorAssignment(binop)),
                _ => output.push(BinaryOperator(binop)),
            }
            continue;
        }
        // === Numerical literals ===
        if c.is_ascii_digit() {
            output.push(LiteralInt);
            while stream.peek().filter(|i| i.is_ascii_digit()).is_some() {
                stream.next();
            }
            continue;
        }
        match c {
            // === Identifiers ===
            _ if c.is_ascii_alphabetic() || c == '_' => {
                consume!(Identifier);
                while stream
                    .peek()
                    .filter(|&&i| i.is_ascii_alphanumeric() || i == '_')
                    .is_some()
                {
                    stream.next();
                }
            }
            // === Special case for binary operator / ===
            //     Needed to handle comments
            '/' => {
                stream.next();
                match stream.peek() {
                    Some('=') => consume!(BinaryOperatorAssignment(Slash)),
                    Some('/') => {
                        while stream.peek().filter(|&&i| i != '\n').is_some() {
                            stream.next();
                        }
                        output.push(Comment);
                    }
                    _ => output.push(BinaryOperator(Slash)),
                }
            }
            // === Structurals ===
            ',' => consume!(Comma),
            ';' => consume!(Semicolon),
            '(' => consume!(Left(Parenthesis)),
            ')' => consume!(Right(Parenthesis)),
            '{' => consume!(Left(Brace)),
            '}' => consume!(Right(Brace)),
            '[' => consume!(Left(Bracket)),
            ']' => consume!(Right(Bracket)),
            '!' => consume!(Exclamation),
            '#' => consume!(Sharp),
            ':' => {
                stream.next();
                match stream.peek() {
                    Some(':') => consume!(DoubleColon),
                    _ => {
                        output.push(Colon);
                    }
                }
                output.push(Comma);
            }
            // === Paired tokens ===

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
                        '=' => consume!(LessEqual),
                        '<' => {
                            stream.next();
                            if stream.peek() == Some(&'=') {
                                consume!(BinaryOperatorAssignment(Shl));
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
                        '=' => consume!(GreaterEqual),
                        '>' => {
                            stream.next();
                            if stream.peek() == Some(&'=') {
                                consume!(BinaryOperatorAssignment(Shr))
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
                    Some('=') => consume!(DoubleEqual),
                    _ => output.push(Equal),
                }
            }
            ' ' | '\n' if c.is_ascii_whitespace() => {
                consume!(Whitespace);
                while stream.peek().filter(|i| i.is_ascii_whitespace()).is_some() {
                    stream.next();
                }
            }

            _ => panic!("Unexpected character {}", c),
        }
    }
    output.push(Eof);

    output
}
