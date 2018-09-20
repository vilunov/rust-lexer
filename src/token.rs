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
    Eof
}

/// Tokenize the stream of incoming source code
///
/// # Panics
///
/// When the tokenizer encounters an unexpected character
pub fn tokenize<S>(mut stream: S) -> Vec<Token>
    where S: Iterator<Item=char> {

    use self::Token::*;
    use self::BinaryOperator::*;

    let mut output = vec![];
    let mut current_char = stream.next();

    while let Some(c) = current_char {
        match c {
            _ if c.is_ascii_digit() => {
                // Skip all digits, we don't store the value anyway
                while let Some(c2) = current_char {
                    if !c2.is_ascii_digit() { break; }

                    current_char = stream.next();
                }
                output.push(LiteralInt);
            }
            '+' => {
                output.push(BinaryOperator(Plus));
                current_char = stream.next();
            }
            _ => panic!("Unexpected character {}", c)
        }
    }
    output.push(Eof);

    output
}
