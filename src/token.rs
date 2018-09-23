use std::iter::Peekable;

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
    /// `?`
    Question,
    /// `$`
    Dollar,
    /// `'`, used for lifetimes, not chars.
    /// This token should never occur, lifetimes use `IdentifierLifetime`
    Quote,
    /// `#`
    Sharp,
    /// `<-`
    LeftArrow,
    /// `->`
    RightArrow,
    /// `=>`
    FatArrow,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `...`
    DotDotDot,
    /// `.=`.
    /// This token should never occur, it is used for macro purposes in rustc
    DotEq,
    /// `..=`
    DotDotEq,

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

    // Literals
    LiteralInt,
    LiteralStr,
    LiteralChar,

    Identifier,
    IdentifierLifetime,

    /// End of stream
    Eof,
}

/// Try to convert a char into a binary operator
/// This function does not output binary operators which are handled by special cases.
fn char_to_binop(c: char) -> Option<BinaryOperator> {
    match c {
        '+' => Some(BinaryOperator::Plus),
        // '-' => Some(BinaryOperator::Minus), // Handled as a special case due to arrows
        '*' => Some(BinaryOperator::Star),
        // '/' => Some(BinaryOperator::Slash), // Handled as a special case due to comments
        '%' => Some(BinaryOperator::Percent),
        '^' => Some(BinaryOperator::Caret),
        '&' => Some(BinaryOperator::And),
        '|' => Some(BinaryOperator::Or),
        _ => None,
    }
}

/// This character is eligible to be identifier's first char
/// https://github.com/rust-lang/rust/blob/af50e3822c4ceda60445c4a2adbb3bfa480ebd39/src/libsyntax/parse/lexer/mod.rs#L1809
fn is_ident_start(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    // || (c > '\x7f' && c.is_xid_start())
}

/// This character is eligible to be identifier's non-first char
fn is_ident_char(c: &&char) -> bool {
    let c = **c;
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
    // || (c > '\x7f' && c.is_xid_continue())
}

/// Tries to read a char from the stream as it would be in literals
///
/// Returns true if succeeded in reading a char
fn read_char<S>(stream: &mut Peekable<S>, delimiter: char) -> bool
where
    S: Iterator<Item = char>,
{
    let val = match stream.peek() {
        Some(&c) if (c == '\t' || c == '\r' || c == '\n' || c == '\'') && delimiter == '\'' => {
            false
        }
        Some('\r') => {
            stream.next();
            stream.peek() == Some(&'\n')
        }
        Some('\\') => {
            stream.next();
            let c = match stream.peek().cloned() {
                Some(c) => c,
                None => return false,
            };
            match c {
                'n' | 'r' | 't' | '\\' | '\'' | '"' | '0' | 'u' | 'x' => true,
                '\n' if delimiter == '"' => {
                    while stream
                        .peek()
                        .cloned()
                        .filter(|i| i.is_ascii_whitespace())
                        .is_some()
                    {
                        stream.next();
                    }
                    true
                }
                _ => false,
            }
        }
        Some(_) => true,
        None => false,
    };
    stream.next();
    val
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
    macro_rules! skip_while {
        ($predicate: expr) => {
            while stream.peek().filter($predicate).is_some() {
                stream.next();
            }
        };
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
        // TODO floats; _; types - u8, f64, etc
        if c.is_ascii_digit() {
            output.push(LiteralInt);
            skip_while!(|i| i.is_ascii_digit());
            continue;
        }
        match c {
            // === Identifiers ===
            _ if is_ident_start(c) => {
                consume!(Identifier);
                skip_while!(is_ident_char);
            }
            // === Special case for operators ===
            // --- Needed to handle comments
            '/' => {
                stream.next();
                match stream.peek() {
                    Some('=') => consume!(BinaryOperatorAssignment(Slash)),
                    Some('*') => {
                        consume!(Comment);
                        while let Some(_) = stream.peek() {
                            skip_while!(|&&i| i != '*');
                            stream.next();
                            if let Some(&'/') = stream.peek() {
                                stream.next();
                                break;
                            }
                        }
                    }
                    Some('/') => {
                        skip_while!(|&&i| i != '\n');
                        output.push(Comment);
                    }
                    _ => output.push(BinaryOperator(Slash)),
                }
            }
            // --- Needed to handle right arrows
            '-' => {
                stream.next();
                match stream.peek() {
                    Some('=') => consume!(BinaryOperatorAssignment(Minus)),
                    Some('>') => consume!(RightArrow),
                    _ => output.push(BinaryOperator(Minus)),
                }
            }
            // === Structurals ===
            ',' => consume!(Comma),
            ';' => consume!(Semicolon),
            '!' => consume!(Exclamation),
            '?' => consume!(Question),
            '$' => consume!(Dollar),
            '#' => consume!(Sharp),
            ':' => {
                stream.next();
                match stream.peek() {
                    Some(':') => consume!(DoubleColon),
                    _ => output.push(Colon),
                }
            }
            '.' => {
                stream.next();
                match stream.peek() {
                    Some('.') => {
                        stream.next();
                        match stream.peek() {
                            Some('.') => consume!(DotDotDot),
                            Some('=') => consume!(DotDotEq),
                            _ => output.push(DotDot),
                        }
                    }
                    //Some('=') => consume!(DotEq), // This token should never occur in real code
                    _ => output.push(Dot),
                }
            }
            // === Lifetimes and character literas ===
            '\'' => {
                stream.next();
                match stream.peek() {
                    // At this point we check whether the first symbol could be the start of lifetime
                    Some(&c) if is_ident_start(c) => {
                        stream.next();
                        match stream.peek().cloned() {
                            // If it is and the next symbol is a single quote, then it is a char literal
                            Some('\'') => consume!(LiteralChar),
                            // If it's not, then it is a lifetime identifier
                            Some(c2) if is_ident_char(&&c2) => {
                                skip_while!(is_ident_char);
                                // Lifetimes can't have a closing quote at the end
                                // The user could mistakenly try to create a char literal with multiple codepoints
                                assert_ne!(
                                    stream.peek(),
                                    Some(&'\''),
                                    "Char literal must have at most one codepoint"
                                );
                                output.push(IdentifierLifetime)
                            }
                            _ => output.push(IdentifierLifetime),
                        }
                    }
                    Some('\'') => panic!("You can't simply put two single quotes in a row"),
                    // The character is not the start of a lifetime identifier, it is a char literal
                    Some(_) => assert!(read_char(&mut stream, '\'')),
                    None => panic!("EOF after opening quote"),
                }
            }
            // === Paired tokens ===
            '(' => consume!(Left(Parenthesis)),
            ')' => consume!(Right(Parenthesis)),
            '{' => consume!(Left(Brace)),
            '}' => consume!(Right(Brace)),
            '[' => consume!(Left(Bracket)),
            ']' => consume!(Right(Bracket)),
            // === String literals ===
            // TODO other types of literals
            '\"' => {
                consume!(LiteralStr);
                while stream.peek() != Some(&'"') {
                    assert!(read_char(&mut stream, '"'));
                }
                stream.next();
            }
            // === Comparison operators ===
            '<' => {
                stream.next();
                match stream.peek() {
                    Some('=') => consume!(LessEqual),
                    Some('-') => consume!(LeftArrow),
                    Some('<') => {
                        stream.next();
                        match stream.peek() {
                            Some('=') => consume!(BinaryOperatorAssignment(Shl)),
                            _ => output.push(BinaryOperator(Shl)),
                        }
                    }
                    _ => output.push(LessThan),
                }
            }
            '>' => {
                stream.next();
                match stream.peek() {
                    Some('=') => consume!(GreaterEqual),
                    Some('>') => {
                        stream.next();
                        match stream.peek() {
                            Some('=') => consume!(BinaryOperatorAssignment(Shr)),
                            _ => output.push(BinaryOperator(Shr)),
                        }
                    }
                    _ => output.push(GreaterThan),
                }
            }
            '=' => {
                stream.next();
                match stream.peek() {
                    Some('=') => consume!(DoubleEqual),
                    Some('>') => consume!(RightArrow),
                    _ => output.push(Equal),
                }
            }
            ' ' | '\n' if c.is_ascii_whitespace() => {
                consume!(Whitespace);
                skip_while!(|i| i.is_ascii_whitespace());
            }

            _ => panic!("Unexpected character {}", c),
        }
    }
    output.push(Eof);

    output
}
