#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    StringLit(String),

    // Identifier
    Ident(String),

    // Keywords
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Return,
    True,
    False,
    And,
    Or,
    Not,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    DotDot,

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,

    // End of file
    Eof,
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.pos >= self.source.len() {
                tokens.push(Token::Eof);
                break;
            }

            let ch = self.source[self.pos];

            // Two-char tokens
            if ch == '=' && self.peek_next() == Some('=') {
                tokens.push(Token::EqEq);
                self.pos += 2;
            } else if ch == '!' && self.peek_next() == Some('=') {
                tokens.push(Token::BangEq);
                self.pos += 2;
            } else if ch == '<' && self.peek_next() == Some('=') {
                tokens.push(Token::LtEq);
                self.pos += 2;
            } else if ch == '>' && self.peek_next() == Some('=') {
                tokens.push(Token::GtEq);
                self.pos += 2;
            } else if ch == '.' && self.peek_next() == Some('.') {
                tokens.push(Token::DotDot);
                self.pos += 2;
            }
            // Single-char tokens
            else if ch == '=' {
                tokens.push(Token::Eq);
                self.pos += 1;
            } else if ch == '+' {
                tokens.push(Token::Plus);
                self.pos += 1;
            } else if ch == '-' {
                tokens.push(Token::Minus);
                self.pos += 1;
            } else if ch == '*' {
                tokens.push(Token::Star);
                self.pos += 1;
            } else if ch == '/' {
                tokens.push(Token::Slash);
                self.pos += 1;
            } else if ch == '%' {
                tokens.push(Token::Percent);
                self.pos += 1;
            } else if ch == '<' {
                tokens.push(Token::Lt);
                self.pos += 1;
            } else if ch == '>' {
                tokens.push(Token::Gt);
                self.pos += 1;
            } else if ch == '(' {
                tokens.push(Token::LParen);
                self.pos += 1;
            } else if ch == ')' {
                tokens.push(Token::RParen);
                self.pos += 1;
            } else if ch == '{' {
                tokens.push(Token::LBrace);
                self.pos += 1;
            } else if ch == '}' {
                tokens.push(Token::RBrace);
                self.pos += 1;
            } else if ch == '[' {
                tokens.push(Token::LBracket);
                self.pos += 1;
            } else if ch == ']' {
                tokens.push(Token::RBracket);
                self.pos += 1;
            } else if ch == ',' {
                tokens.push(Token::Comma);
                self.pos += 1;
            }
            // Number literals
            else if ch.is_ascii_digit() {
                tokens.push(self.read_number()?);
            }
            // String literals
            else if ch == '"' {
                tokens.push(self.read_string()?);
            }
            // Identifiers and keywords
            else if ch.is_ascii_alphabetic() || ch == '_' {
                tokens.push(self.read_ident());
            } else {
                return Err(format!("Unexpected character '{}' at line {}", ch, self.line));
            }
        }

        Ok(tokens)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.source.len() {
            let ch = self.source[self.pos];
            if ch == '\n' {
                self.line += 1;
                self.pos += 1;
            } else if ch.is_ascii_whitespace() {
                self.pos += 1;
            } else if ch == '#' {
                // Skip to end of line
                while self.pos < self.source.len() && self.source[self.pos] != '\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let start = self.pos;
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos < self.source.len() && self.source[self.pos] == '.' {
            // Check it's not `..`
            if self.peek_next() != Some('.') {
                self.pos += 1; // consume '.'
                while self.pos < self.source.len() && self.source[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
            }
        }
        let text: String = self.source[start..self.pos].iter().collect();
        let num: f64 = text
            .parse()
            .map_err(|_| format!("Invalid number '{}' at line {}", text, self.line))?;
        Ok(Token::Number(num))
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.pos += 1; // skip opening quote
        let mut s = String::new();
        while self.pos < self.source.len() && self.source[self.pos] != '"' {
            if self.source[self.pos] == '\n' {
                self.line += 1;
            }
            s.push(self.source[self.pos]);
            self.pos += 1;
        }
        if self.pos >= self.source.len() {
            return Err(format!("Unterminated string at line {}", self.line));
        }
        self.pos += 1; // skip closing quote
        Ok(Token::StringLit(s))
    }

    fn read_ident(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.source.len()
            && (self.source[self.pos].is_ascii_alphanumeric() || self.source[self.pos] == '_')
        {
            self.pos += 1;
        }
        let text: String = self.source[start..self.pos].iter().collect();
        match text.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Ident(text),
        }
    }
}
