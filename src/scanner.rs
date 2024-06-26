use std::iter::Peekable;
use std::str::Chars;
use thiserror::Error;

pub struct Scanner<'a> {
    current: Peekable<Chars<'a>>,
    line: usize,
    position: usize,
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unrecognized character")]
    UnrecognizedCharacter,

    #[error("Unterminated string")]
    UnterminatedString,
}

#[derive(Debug)]
pub enum KeywordKind {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

// All tokens return their starting column position
#[derive(Debug)]
pub enum Token {
    // Single character tokens
    LeftParen(usize),
    RightParen(usize),
    LeftBrace(usize),
    RightBrace(usize),
    Comma(usize),
    Dot(usize),
    Minus(usize),
    Plus(usize),
    Semicolon(usize),
    Slash(usize),
    Star(usize),

    // One or two character tokens
    Bang(usize),
    BangEqual(usize),
    Equal(usize),
    EqualEqual(usize),
    Greater(usize),
    GreaterEqual(usize),
    Less(usize),
    LessEqual(usize),

    // Literals
    Identifier(usize, usize),
    String(usize, usize),
    Number(usize, usize),
    Keyword(usize, KeywordKind),

    Error(usize),
    EOF(usize),
}

macro_rules! scan_two_char_operator {
    ($self:expr, $one_result:expr, $two_result:expr) => {
        if let Some(c) = $self.current.peek() {
            if *c == '=' {
                $self.advance();
                return Ok($two_result($self.position));
            } else {
                return Ok($one_result($self.position));
            }
        } else {
            return Ok($one_result($self.position));
        }
    };
}

impl<'a, 'outlives_a: 'a> Scanner<'a> {
    pub fn new(source: &'outlives_a str) -> Self {
        Self {
            current: source.chars().peekable(),
            line: 1,
            position: 1,
        }
    }

    pub fn scan_token(&'a mut self) -> Result<Token, ScannerError> {
        if let Some(c) = self.advance() {
            return match c {
                '(' => Ok(Token::LeftParen(self.position)),
                ')' => Ok(Token::RightParen(self.position)),
                '{' => Ok(Token::LeftBrace(self.position)),
                '}' => Ok(Token::RightBrace(self.position)),
                ';' => Ok(Token::Semicolon(self.position)),
                ',' => Ok(Token::Comma(self.position)),
                '.' => Ok(Token::Dot(self.position)),
                '-' => Ok(Token::Minus(self.position)),
                '+' => Ok(Token::Plus(self.position)),
                '/' => Ok(Token::Slash(self.position)),
                '*' => Ok(Token::Star(self.position)),
                '!' => scan_two_char_operator!(self, Token::Bang, Token::BangEqual),
                '=' => scan_two_char_operator!(self, Token::Equal, Token::EqualEqual),
                '<' => scan_two_char_operator!(self, Token::Less, Token::LessEqual),
                '>' => scan_two_char_operator!(self, Token::Greater, Token::GreaterEqual),
                '"' => self.scan_string(),
                c if c.is_ascii_digit() || c == '_' => self.scan_number(),
                c if c.is_ascii_alphabetic() => self.scan_identifier(),
                _ => return Err(ScannerError::UnrecognizedCharacter)
            };
        }

        if self.current.peek().is_none() {
            return Ok(Token::EOF(self.position));
        }

        return Err(ScannerError::UnrecognizedCharacter);
    }

    fn is_at_end(&self) -> bool {
        self.current.clone().peek().is_some()
    }

    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.current.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.current.peek()
    }

    fn peek_next(&mut self) -> Option<char> {
        let mut peeked_iter = self.current.clone();
        peeked_iter.next();
        peeked_iter.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c.is_ascii_whitespace() {
                // Skip whitespace
                self.advance();
            } else if *c == '\n' {
                // Skip newline
                self.line += 1;
                self.advance();
            } else if *c == '/' {
                if self.peek_next().is_some_and(|c| c == '/') {
                    // Skip entire comment line
                    while self.peek().is_some_and(|c| *c != '\n') && !self.is_at_end() {
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
    }

    fn scan_string(&'a mut self) -> Result<Token, ScannerError> {
        let start_position = self.position;

        while self.peek().is_some_and(|c| *c != '"') && !self.is_at_end() {
            if self.peek().is_some_and(|c| *c == '\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString);
        }

        // Consume closing quote
        self.advance();

        Ok(Token::String(start_position, self.position))
    }

    fn scan_number(&mut self) -> Result<Token, ScannerError> {
        let start_position = self.position;

        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek().is_some_and(|c| *c == '.')
            && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance(); // Consume dot

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        Ok(Token::Number(start_position, self.position))
    }

    fn scan_identifier(&mut self) -> Result<Token, ScannerError> {
        let start_position = self.position;
        let mut buffer = String::new();

        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_') {
            buffer.push(*self.peek().unwrap());
            self.advance();
        }

        match buffer.as_str() {
            "and" => return Ok(Token::Keyword(start_position, KeywordKind::And)),
            "class" => return Ok(Token::Keyword(start_position, KeywordKind::Class)),
            "else" => return Ok(Token::Keyword(start_position, KeywordKind::Else)),
            "false" => return Ok(Token::Keyword(start_position, KeywordKind::False)),
            "for" => return Ok(Token::Keyword(start_position, KeywordKind::For)),
            "fun" => return Ok(Token::Keyword(start_position, KeywordKind::Fun)),
            "if" => return Ok(Token::Keyword(start_position, KeywordKind::If)),
            "nil" => return Ok(Token::Keyword(start_position, KeywordKind::Nil)),
            "or" => return Ok(Token::Keyword(start_position, KeywordKind::Or)),
            "print" => return Ok(Token::Keyword(start_position, KeywordKind::Print)),
            "return" => return Ok(Token::Keyword(start_position, KeywordKind::Return)),
            "super" => return Ok(Token::Keyword(start_position, KeywordKind::Super)),
            "this" => return Ok(Token::Keyword(start_position, KeywordKind::This)),
            "true" => return Ok(Token::Keyword(start_position, KeywordKind::True)),
            "var" => return Ok(Token::Keyword(start_position, KeywordKind::Var)),
            "while" => return Ok(Token::Keyword(start_position, KeywordKind::While)),
            _ => {}
        }

        Ok(Token::Identifier(start_position, self.position))
    }
}
