use std::collections::HashMap;

use crate::parser::{BinOp, Expr, Stmt, UnaryOp};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Array(Vec<Value>),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if *n == (*n as i64 as f64) {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(elems) => {
                write!(f, "[")?;
                for (i, v) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Function { .. } => write!(f, "<function>"),
            Value::Null => write!(f, "null"),
        }
    }
}

enum Signal {
    None,
    Return(Value),
}

pub struct Interpreter {
    scopes: Vec<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            scopes: vec![HashMap::new()],
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn get_var(&self, name: &str) -> Result<Value, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        Err(format!("Undefined variable '{}'", name))
    }

    fn set_var(&mut self, name: &str, val: Value) {
        // Set in the nearest scope that has it, or current scope
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return;
            }
        }
        // New variable in current (top) scope
        self.scopes.last_mut().unwrap().insert(name.to_string(), val);
    }

    fn define_var(&mut self, name: String, val: Value) {
        self.scopes.last_mut().unwrap().insert(name, val);
    }

    pub fn run(&mut self, program: &[Stmt]) -> Result<(), String> {
        for stmt in program {
            if let Signal::Return(_) = self.exec_stmt(stmt)? {
                break;
            }
        }
        Ok(())
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Signal, String> {
        match stmt {
            Stmt::Let(name, expr) => {
                let val = self.eval_expr(expr)?;
                self.define_var(name.clone(), val);
            }
            Stmt::Assign(name, expr) => {
                let val = self.eval_expr(expr)?;
                self.set_var(name, val);
            }
            Stmt::IndexAssign(name, index_expr, value_expr) => {
                let idx = self.eval_expr(index_expr)?;
                let val = self.eval_expr(value_expr)?;
                let i = match idx {
                    Value::Number(n) => n as usize,
                    _ => return Err("Array index must be a number".to_string()),
                };
                // Find and mutate the array in-place
                for scope in self.scopes.iter_mut().rev() {
                    if let Some(arr_val) = scope.get_mut(name) {
                        match arr_val {
                            Value::Array(elems) => {
                                if i >= elems.len() {
                                    return Err(format!("Index {} out of bounds", i));
                                }
                                elems[i] = val;
                                return Ok(Signal::None);
                            }
                            _ => return Err(format!("'{}' is not an array", name)),
                        }
                    }
                }
                return Err(format!("Undefined variable '{}'", name));
            }
            Stmt::If(cond, body, else_body) => {
                let val = self.eval_expr(cond)?;
                if Self::is_truthy(&val) {
                    let sig = self.exec_block(body)?;
                    if let Signal::Return(_) = sig {
                        return Ok(sig);
                    }
                } else if let Some(else_b) = else_body {
                    let sig = self.exec_block(else_b)?;
                    if let Signal::Return(_) = sig {
                        return Ok(sig);
                    }
                }
            }
            Stmt::While(cond, body) => {
                loop {
                    let val = self.eval_expr(cond)?;
                    if !Self::is_truthy(&val) {
                        break;
                    }
                    let sig = self.exec_block(body)?;
                    if let Signal::Return(_) = sig {
                        return Ok(sig);
                    }
                }
            }
            Stmt::For(var, start_expr, end_expr, body) => {
                let start = match self.eval_expr(start_expr)? {
                    Value::Number(n) => n as i64,
                    _ => return Err("For range start must be a number".to_string()),
                };
                let end = match self.eval_expr(end_expr)? {
                    Value::Number(n) => n as i64,
                    _ => return Err("For range end must be a number".to_string()),
                };
                for i in start..end {
                    self.push_scope();
                    self.define_var(var.clone(), Value::Number(i as f64));
                    for s in body {
                        let sig = self.exec_stmt(s)?;
                        if let Signal::Return(_) = sig {
                            self.pop_scope();
                            return Ok(sig);
                        }
                    }
                    self.pop_scope();
                }
            }
            Stmt::Fn(name, params, body) => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                };
                self.define_var(name.clone(), func);
            }
            Stmt::Return(expr) => {
                let val = match expr {
                    Some(e) => self.eval_expr(e)?,
                    None => Value::Null,
                };
                return Ok(Signal::Return(val));
            }
            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr)?;
            }
        }
        Ok(Signal::None)
    }

    fn exec_block(&mut self, stmts: &[Stmt]) -> Result<Signal, String> {
        self.push_scope();
        for stmt in stmts {
            let sig = self.exec_stmt(stmt)?;
            if let Signal::Return(_) = sig {
                self.pop_scope();
                return Ok(sig);
            }
        }
        self.pop_scope();
        Ok(Signal::None)
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::StringLit(s) => Ok(Value::Str(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Ident(name) => self.get_var(name),
            Expr::Array(elems) => {
                let mut vals = Vec::new();
                for e in elems {
                    vals.push(self.eval_expr(e)?);
                }
                Ok(Value::Array(vals))
            }
            Expr::Index(arr_expr, idx_expr) => {
                let arr = self.eval_expr(arr_expr)?;
                let idx = self.eval_expr(idx_expr)?;
                match (arr, idx) {
                    (Value::Array(elems), Value::Number(n)) => {
                        let i = n as usize;
                        if i >= elems.len() {
                            return Err(format!("Index {} out of bounds", i));
                        }
                        Ok(elems[i].clone())
                    }
                    _ => Err("Index operator requires array and number".to_string()),
                }
            }
            Expr::Call(func_expr, args) => {
                // Check for built-in functions
                if let Expr::Ident(name) = func_expr.as_ref() {
                    match name.as_str() {
                        "print" => {
                            let mut vals = Vec::new();
                            for a in args {
                                vals.push(self.eval_expr(a)?);
                            }
                            if let Some(v) = vals.first() {
                                println!("{}", v);
                            }
                            return Ok(Value::Null);
                        }
                        "len" => {
                            if args.len() != 1 {
                                return Err("len() takes exactly 1 argument".to_string());
                            }
                            let val = self.eval_expr(&args[0])?;
                            return match val {
                                Value::Array(elems) => Ok(Value::Number(elems.len() as f64)),
                                Value::Str(s) => Ok(Value::Number(s.len() as f64)),
                                _ => Err("len() requires array or string".to_string()),
                            };
                        }
                        _ => {}
                    }
                }

                let func = self.eval_expr(func_expr)?;
                let mut arg_vals = Vec::new();
                for a in args {
                    arg_vals.push(self.eval_expr(a)?);
                }

                match func {
                    Value::Function { params, body } => {
                        if params.len() != arg_vals.len() {
                            return Err(format!(
                                "Expected {} arguments, got {}",
                                params.len(),
                                arg_vals.len()
                            ));
                        }
                        self.push_scope();
                        for (p, v) in params.iter().zip(arg_vals) {
                            self.define_var(p.clone(), v);
                        }
                        let mut result = Value::Null;
                        for stmt in &body {
                            match self.exec_stmt(stmt)? {
                                Signal::Return(val) => {
                                    result = val;
                                    break;
                                }
                                Signal::None => {}
                            }
                        }
                        self.pop_scope();
                        Ok(result)
                    }
                    _ => Err("Attempted to call a non-function".to_string()),
                }
            }
            Expr::Unary(op, operand) => {
                let val = self.eval_expr(operand)?;
                match op {
                    UnaryOp::Neg => match val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err("Unary '-' requires a number".to_string()),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!Self::is_truthy(&val))),
                }
            }
            Expr::Binary(left, op, right) => {
                // Short-circuit for and/or
                if matches!(op, BinOp::And) {
                    let lv = self.eval_expr(left)?;
                    if !Self::is_truthy(&lv) {
                        return Ok(lv);
                    }
                    return self.eval_expr(right);
                }
                if matches!(op, BinOp::Or) {
                    let lv = self.eval_expr(left)?;
                    if Self::is_truthy(&lv) {
                        return Ok(lv);
                    }
                    return self.eval_expr(right);
                }

                let lv = self.eval_expr(left)?;
                let rv = self.eval_expr(right)?;

                match op {
                    BinOp::Add => match (lv, rv) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                        (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
                        (Value::Array(mut a), Value::Array(b)) => {
                            a.extend(b);
                            Ok(Value::Array(a))
                        }
                        _ => Err("'+' requires two numbers, two strings, or two arrays".to_string()),
                    },
                    BinOp::Sub => Self::num_op(lv, rv, |a, b| a - b),
                    BinOp::Mul => Self::num_op(lv, rv, |a, b| a * b),
                    BinOp::Div => Self::num_op(lv, rv, |a, b| a / b),
                    BinOp::Mod => Self::num_op(lv, rv, |a, b| a % b),
                    BinOp::Lt => Self::cmp_op(lv, rv, |a, b| a < b),
                    BinOp::LtEq => Self::cmp_op(lv, rv, |a, b| a <= b),
                    BinOp::Gt => Self::cmp_op(lv, rv, |a, b| a > b),
                    BinOp::GtEq => Self::cmp_op(lv, rv, |a, b| a >= b),
                    BinOp::Eq => Ok(Value::Bool(Self::values_equal(&lv, &rv))),
                    BinOp::Neq => Ok(Value::Bool(!Self::values_equal(&lv, &rv))),
                    BinOp::And | BinOp::Or => unreachable!(),
                }
            }
        }
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Function { .. } => true,
        }
    }

    fn num_op(lv: Value, rv: Value, f: fn(f64, f64) -> f64) -> Result<Value, String> {
        match (lv, rv) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(f(a, b))),
            _ => Err("Arithmetic operator requires two numbers".to_string()),
        }
    }

    fn cmp_op(lv: Value, rv: Value, f: fn(f64, f64) -> bool) -> Result<Value, String> {
        match (lv, rv) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(f(a, b))),
            _ => Err("Comparison operator requires two numbers".to_string()),
        }
    }

    fn values_equal(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Str(x), Value::Str(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
