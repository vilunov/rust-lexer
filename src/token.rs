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
fn is_ident_char(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
    // || (c > '\x7f' && c.is_xid_continue())
}

pub struct Tokenizer<S> {
    iter: S,
    pos: usize,
    cur: Option<char>,
}

impl<S> Tokenizer<S>
where
    S: Iterator<Item = char>,
{
    pub fn new(mut iter: S) -> Self {
        let cur = iter.next();
        Self { iter, pos: 0, cur }
    }

    fn adv(&mut self) {
        self.cur = self.iter.next();
        self.pos += 1;
    }

    fn skip_chars<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(c) = self.cur {
            if !predicate(c) {
                break;
            }
            self.adv();
        }
    }

    fn skip_whitespace(&mut self) {
        self.skip_chars(|i| i.is_ascii_whitespace());
    }

    fn next(&mut self) -> Option<char> {
        self.adv();
        self.cur
    }

    /// Tries to read a char from the stream as it would be in literals
    ///
    /// Returns true if succeeded in reading a char
    fn read_char(&mut self, delimiter: char) -> bool {
        let val = match self.cur {
            Some(c) if (c == '\t' || c == '\r' || c == '\n' || c == '\'') && delimiter == '\'' => {
                false
            }
            Some('\r') => self.next() == Some('\n'),
            Some('\\') => {
                let c = match self.next() {
                    Some(c) => c,
                    None => return false,
                };
                match c {
                    'n' | 'r' | 't' | '\\' | '\'' | '"' | '0' => true,
                    'u' => {
                        assert_eq!(self.next(), Some('{'));
                        self.adv();
                        self.skip_chars(|c| c.is_ascii_hexdigit());
                        assert_eq!(self.cur, Some('}'));
                        true
                    }
                    'x' => {
                        assert!(self.next().unwrap().is_ascii_hexdigit());
                        assert!(self.next().unwrap().is_ascii_hexdigit());
                        true
                    }
                    '\n' if delimiter == '"' => {
                        self.skip_whitespace();
                        true
                    }
                    _ => false,
                }
            }
            Some(_) => true,
            None => false,
        };
        self.adv();
        val
    }
}

impl<S> Iterator for Tokenizer<S>
where
    S: Iterator<Item = char>,
{
    type Item = Token;

    /// Retrieve the next token of incoming source code
    ///
    /// # Panics
    ///
    /// When the tokenizer encounters an unexpected character
    fn next(&mut self) -> Option<Token> {
        use self::BinaryOperator::*;
        use self::PairedToken::*;
        use self::Token::*;

        macro_rules! consume {
            ($token: expr) => {{
                self.adv();
                $token
            }};
        }

        let cur = match self.cur {
            Some(c) => c,
            None => return None,
        };

        // === Binary operators ===
        if let Some(binop) = char_to_binop(cur) {
            return Some(match self.next() {
                Some('=') => consume!(BinaryOperatorAssignment(binop)),
                _ => BinaryOperator(binop),
            });
        }
        // === Numerical literals ===
        // TODO floats; _; types - u8, f64, etc
        if cur.is_ascii_digit() {
            self.skip_chars(|i| i.is_ascii_digit());
            return Some(LiteralInt);
        }
        // === Identifiers ===
        if is_ident_start(cur) {
            self.skip_chars(is_ident_char);
            return Some(Identifier);
        }

        Some(match cur {
            // === Special case for operators ===
            // --- Needed to handle comments
            '/' => {
                match self.next() {
                    Some('=') => consume!(BinaryOperatorAssignment(Slash)),
                    // Block comments
                    Some('*') => {
                        self.adv();
                        while let Some(_) = self.cur {
                            self.skip_chars(|i| i != '*');
                            self.adv();
                            if let Some('/') = self.cur {
                                self.adv();
                                break;
                            }
                        }
                        Comment
                    }
                    // Line comments
                    Some('/') => {
                        self.skip_chars(|i| i != '\n');
                        Comment
                    }
                    _ => BinaryOperator(Slash),
                }
            }
            // --- Needed to handle right arrows
            '-' => match self.next() {
                Some('=') => consume!(BinaryOperatorAssignment(Minus)),
                Some('>') => consume!(RightArrow),
                _ => BinaryOperator(Minus),
            },
            // === Structurals ===
            ',' => consume!(Comma),
            ';' => consume!(Semicolon),
            '!' => consume!(Exclamation),
            '?' => consume!(Question),
            '$' => consume!(Dollar),
            '#' => consume!(Sharp),
            ':' => match self.next() {
                Some(':') => consume!(DoubleColon),
                _ => Colon,
            },
            '.' => {
                match self.next() {
                    Some('.') => match self.next() {
                        Some('.') => consume!(DotDotDot),
                        Some('=') => consume!(DotDotEq),
                        _ => DotDot,
                    },
                    //Some('=') => consume!(DotEq), // This token should never occur in real code
                    _ => Dot,
                }
            }
            // === Lifetimes and character literals ===
            '\'' => {
                match self.next() {
                    // At this point we check whether the first symbol could be the start of lifetime
                    Some(c) if is_ident_start(c) => {
                        match self.next() {
                            // If it is and the next symbol is a single quote, then it is a char literal
                            Some('\'') => consume!(LiteralChar),
                            // If it's not, then it is a lifetime identifier
                            Some(c2) if is_ident_char(c2) => {
                                self.skip_chars(is_ident_char);
                                // Lifetimes can't have a closing quote at the end
                                // The user could mistakenly try to create a char literal with multiple codepoints
                                assert_ne!(
                                    self.cur,
                                    Some('\''),
                                    "Char literal must have at most one codepoint"
                                );
                                IdentifierLifetime
                            }
                            _ => IdentifierLifetime,
                        }
                    }
                    Some('\'') => panic!("You can't simply put two single quotes in a row"),
                    // The character is not the start of a lifetime identifier, it is a char literal
                    Some(_) => {
                        assert!(self.read_char('\''));
                        assert_eq!(self.cur, Some('\''), "Expected single quote");
                        self.next();
                        LiteralChar
                    }
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
                self.adv();
                while self.cur != Some('"') {
                    assert!(self.read_char('"'));
                }
                self.adv();
                LiteralStr
            }
            // === Comparison operators and assignment ===
            '<' => match self.next() {
                Some('=') => consume!(LessEqual),
                Some('-') => consume!(LeftArrow),
                Some('<') => match self.next() {
                    Some('=') => consume!(BinaryOperatorAssignment(Shl)),
                    _ => BinaryOperator(Shl),
                },
                _ => LessThan,
            },
            '>' => match self.next() {
                Some('=') => consume!(GreaterEqual),
                Some('>') => match self.next() {
                    Some('=') => consume!(BinaryOperatorAssignment(Shr)),
                    _ => BinaryOperator(Shr),
                },
                _ => GreaterThan,
            },
            '=' => match self.next() {
                Some('=') => consume!(DoubleEqual),
                Some('>') => consume!(RightArrow),
                _ => Equal,
            },
            _ if cur.is_ascii_whitespace() => {
                self.skip_whitespace();
                Whitespace
            }
            _ => panic!("Unexpected character {} at location {}", cur, self.pos),
        })
    }
}
