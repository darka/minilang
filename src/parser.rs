use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    StringLit(String),
    Bool(bool),
    Ident(String),
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Assign(String, Expr),
    IndexAssign(String, Expr, Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    For(String, Expr, Expr, Vec<Stmt>),
    Fn(String, Vec<String>, Vec<Stmt>),
    Return(Option<Expr>),
    ExprStmt(Expr),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.peek() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.peek()))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while *self.peek() != Token::Eof {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Fn => self.parse_fn(),
            Token::Return => self.parse_return(),
            Token::Ident(_) => {
                // Could be assign, index assign, or expr stmt
                self.parse_assign_or_expr()
            }
            _ => {
                let expr = self.parse_expr()?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'let'
        let name = match self.advance() {
            Token::Ident(n) => n,
            t => return Err(format!("Expected identifier after 'let', got {:?}", t)),
        };
        self.expect(&Token::Eq)?;
        let expr = self.parse_expr()?;
        Ok(Stmt::Let(name, expr))
    }

    fn parse_assign_or_expr(&mut self) -> Result<Stmt, String> {
        let name = if let Token::Ident(n) = self.peek() {
            n.clone()
        } else {
            let expr = self.parse_expr()?;
            return Ok(Stmt::ExprStmt(expr));
        };

        // Look ahead for `=` or `[`
        match &self.tokens[self.pos + 1] {
            Token::Eq => {
                self.advance(); // consume ident
                self.advance(); // consume '='
                let expr = self.parse_expr()?;
                Ok(Stmt::Assign(name, expr))
            }
            Token::LBracket => {
                // Check if it's index assign: ident '[' expr ']' '='
                // We need to check further ahead, but let's try parsing as index assign
                // and fall back to expr stmt
                let saved = self.pos;
                self.advance(); // consume ident
                self.advance(); // consume '['
                let index_expr = self.parse_expr()?;
                if *self.peek() == Token::RBracket {
                    self.advance(); // consume ']'
                    if *self.peek() == Token::Eq {
                        self.advance(); // consume '='
                        let value = self.parse_expr()?;
                        return Ok(Stmt::IndexAssign(name, index_expr, value));
                    }
                }
                // Not an index assign, backtrack and parse as expr stmt
                self.pos = saved;
                let expr = self.parse_expr()?;
                Ok(Stmt::ExprStmt(expr))
            }
            _ => {
                let expr = self.parse_expr()?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'if'
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        let else_body = if *self.peek() == Token::Else {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Stmt::If(cond, body, else_body))
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'while'
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While(cond, body))
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'for'
        let var = match self.advance() {
            Token::Ident(n) => n,
            t => return Err(format!("Expected identifier after 'for', got {:?}", t)),
        };
        self.expect(&Token::In)?;
        let start = self.parse_expr()?;
        self.expect(&Token::DotDot)?;
        let end = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::For(var, start, end, body))
    }

    fn parse_fn(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'fn'
        let name = match self.advance() {
            Token::Ident(n) => n,
            t => return Err(format!("Expected function name, got {:?}", t)),
        };
        self.expect(&Token::LParen)?;
        let mut params = Vec::new();
        if *self.peek() != Token::RParen {
            match self.advance() {
                Token::Ident(p) => params.push(p),
                t => return Err(format!("Expected parameter name, got {:?}", t)),
            }
            while *self.peek() == Token::Comma {
                self.advance();
                match self.advance() {
                    Token::Ident(p) => params.push(p),
                    t => return Err(format!("Expected parameter name, got {:?}", t)),
                }
            }
        }
        self.expect(&Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::Fn(name, params, body))
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'return'
        // If the next token could start an expression, parse it
        let expr = match self.peek() {
            Token::RBrace | Token::Eof => None,
            _ => Some(self.parse_expr()?),
        };
        Ok(Stmt::Return(expr))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(&Token::LBrace)?;
        let mut stmts = Vec::new();
        while *self.peek() != Token::RBrace {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&Token::RBrace)?;
        Ok(stmts)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_logic()
    }

    fn parse_logic(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while matches!(self.peek(), Token::And | Token::Or) {
            let op = match self.advance() {
                Token::And => BinOp::And,
                Token::Or => BinOp::Or,
                _ => unreachable!(),
            };
            let right = self.parse_equality()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_compare()?;
        while matches!(self.peek(), Token::EqEq | Token::BangEq) {
            let op = match self.advance() {
                Token::EqEq => BinOp::Eq,
                Token::BangEq => BinOp::Neq,
                _ => unreachable!(),
            };
            let right = self.parse_compare()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), Token::Lt | Token::LtEq | Token::Gt | Token::GtEq) {
            let op = match self.advance() {
                Token::Lt => BinOp::Lt,
                Token::LtEq => BinOp::LtEq,
                Token::Gt => BinOp::Gt,
                Token::GtEq => BinOp::GtEq,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), Token::Star | Token::Slash | Token::Percent) {
            let op = match self.advance() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryOp::Neg, Box::new(expr)))
            }
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryOp::Not, Box::new(expr)))
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek() {
                Token::LParen => {
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if *self.peek() != Token::RParen {
                        args.push(self.parse_expr()?);
                        while *self.peek() == Token::Comma {
                            self.advance();
                            args.push(self.parse_expr()?);
                        }
                    }
                    self.expect(&Token::RParen)?;
                    expr = Expr::Call(Box::new(expr), args);
                }
                Token::LBracket => {
                    self.advance(); // consume '['
                    let index = self.parse_expr()?;
                    self.expect(&Token::RBracket)?;
                    expr = Expr::Index(Box::new(expr), Box::new(index));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Token::Ident(name) => {
                self.advance();
                Ok(Expr::Ident(name))
            }
            Token::LBracket => {
                self.advance(); // consume '['
                let mut elems = Vec::new();
                if *self.peek() != Token::RBracket {
                    elems.push(self.parse_expr()?);
                    while *self.peek() == Token::Comma {
                        self.advance();
                        elems.push(self.parse_expr()?);
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(Expr::Array(elems))
            }
            Token::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            t => Err(format!("Unexpected token {:?}", t)),
        }
    }
}
