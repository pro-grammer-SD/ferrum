use anyhow::{anyhow, Result};

use crate::runtime::{Env, Function};

/// Statement types in the Ferrum AST
/// 
/// Ferrum supports the following statement types:
/// - Print: Output to stdout
/// - Assign: Variable assignment
/// - Expr: Standalone expression evaluation
/// - If/Elif/Else: Conditional execution
/// - While: Looping construct
/// - For: Iteration over sequences
/// - Def: Function definition
/// - Class: Class definition
/// - Block: Generic code blocks (try/except)
/// - Return: Return from function
/// - Import: Module import
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Stmt {
    Print(String),
    Assign(String, String),
    Expr(String),
    If(Vec<(String, Vec<Stmt>)>, Vec<Stmt>),
    Else(Vec<Stmt>),
    While(String, Vec<Stmt>),
    For(String, Vec<Stmt>),
    Def(String, Vec<Stmt>),
    Class(String, Vec<Stmt>),
    Block(String, Vec<Stmt>),
    Return(String),
    Import(String),
}

pub fn run_stmts(stmts: &Vec<Stmt>, env: &mut Env) -> Result<()> {
    for s in stmts.iter() {
        exec_stmt(s, env)?;
    }
    Ok(())
}

/// Public interface to evaluate an expression and return its value
/// Used by REPL for direct expression evaluation
/// 
/// # Arguments
/// * `expr` - Expression string (e.g., "1+2", "x*5", "sin(3.14)")
/// * `env` - Runtime environment with variables and functions
/// 
/// # Returns
/// The computed value or an error
/// 
/// # Example
/// ```ignore
/// let mut env = Env::new();
/// let result = eval_expr_str("1 + 2", &mut env)?;
/// println!("{}", result.to_string()); // prints: 3
/// ```
pub fn eval_expr_str(expr: &str, env: &mut Env) -> Result<crate::runtime::Value> {
    eval_expr(expr, env)
}

fn exec_stmt(stmt: &Stmt, env: &mut Env) -> Result<Option<crate::runtime::Value>> {
    match stmt {
        Stmt::Print(expr) => {
            let v = eval_expr(expr, env)?;
            println!("{}", v.to_string());
            Ok(None)
        }
        Stmt::Assign(lhs, rhs) => {
            let v = eval_expr(rhs, env)?;
            if lhs.contains('.') {
                if let Some((obj, attr)) = lhs.split_once('.') {
                    if env.set_attr(obj, attr, v.clone()).is_ok() {
                        return Ok(None);
                    }
                }
            }
            env.set(lhs.clone(), v);
            Ok(None)
        }
        Stmt::Expr(expr) => {
            let _ = eval_expr(expr, env)?;
            Ok(None)
        }
        Stmt::If(branches, else_block) => {
            for (cond, body) in branches.iter() {
                let v = eval_expr(cond, env)?;
                if v.is_truthy() {
                    run_stmts(body, env)?;
                    return Ok(None);
                }
            }
            if !else_block.is_empty() {
                run_stmts(else_block, env)?;
            }
            Ok(None)
        }
        Stmt::Else(body) => {
            run_stmts(body, env)?;
            Ok(None)
        }
        Stmt::While(cond, body) => {
            while eval_expr(cond, env)?.is_truthy() {
                run_stmts(body, env)?;
            }
            Ok(None)
        }
        Stmt::For(header, body) => {
            // header like "x in range(0,10)" or "x in mylist"
            if let Some((var, rest)) = header.split_once(" in ") {
                let var = var.trim();
                let iter = eval_expr(rest.trim().to_string().as_str(), env)?;
                match iter {
                    crate::runtime::Value::List(vs) => {
                        for val in vs {
                            env.set(var.to_string(), val.clone());
                            run_stmts(body, env)?;
                        }
                    }
                    crate::runtime::Value::Range(start, end) => {
                        for i in start..end {
                            env.set(var.to_string(), crate::runtime::Value::Int(i as i64));
                            run_stmts(body, env)?;
                        }
                    }
                    _ => {
                        return Err(anyhow!("for-loop over non-iterable"));
                    }
                }
            }
            Ok(None)
        }
        Stmt::Def(sig, body) => {
            // sig like: name(a, b=1)
            if let Some((name, args)) = sig.split_once('(') {
                let name = name.trim().to_string();
                let args = args.trim_end_matches(')').to_string();
                let args_vec: Vec<String> = args
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                let func = Function::User {
                    params: args_vec,
                    body: body.clone(),
                };
                env.set_func(name, func);
            }
            Ok(None)
        }
        Stmt::Class(name, body) => {
            // collect methods defined inside class body
            let mut methods = std::collections::HashMap::new();
            for s in body.iter() {
                if let Stmt::Def(sig, mbody) = s {
                    if let Some((mname, args)) = sig.split_once('(') {
                        let mname = mname.trim().to_string();
                        let args = args.trim_end_matches(')').to_string();
                        let args_vec: Vec<String> = args
                            .split(',')
                            .map(|ss| ss.trim().to_string())
                            .filter(|ss| !ss.is_empty())
                            .collect();
                        let func = Function::User { params: args_vec, body: mbody.clone() };
                        methods.insert(mname, func);
                    }
                }
            }
            env.set(name.clone(), crate::runtime::Value::Class(name.clone(), methods));
            Ok(None)
        }
        Stmt::Return(expr) => {
            let v = eval_expr(expr, env)?;
            Ok(Some(v))
        }
        Stmt::Import(name) => {
            env.import_module(name)?;
            Ok(None)
        }
        _ => Ok(None),
    }
}

fn eval_expr(expr: &str, env: &mut Env) -> Result<crate::runtime::Value> {
    let s = expr.trim();
    // helper: split an expression by a top-level operator (not inside parentheses or quotes)
    fn split_top_level(s: &str, op: char) -> Option<(&str, &str)> {
        let mut depth = 0usize;
        let mut in_quote = false;
        let mut prev = '\0';
        for (i, c) in s.char_indices() {
            if c == '"' && prev != '\\' {
                in_quote = !in_quote;
            }
            if in_quote { prev = c; continue; }
            if c == '(' { depth += 1; }
            else if c == ')' { if depth > 0 { depth -= 1; } }
            else if c == op && depth == 0 {
                let (l, r) = s.split_at(i);
                return Some((l.trim(), r[1..].trim()));
            }
            prev = c;
        }
        None
    }

    // binary + handling (string concat or numeric add)
    if let Some((left, right)) = split_top_level(s, '+') {
        let lv = eval_expr(left, env)?;
        let rv = eval_expr(right, env)?;
        match (lv, rv) {
            (crate::runtime::Value::Str(a), crate::runtime::Value::Str(b)) => return Ok(crate::runtime::Value::Str(a + &b)),
            (crate::runtime::Value::Str(a), other) => return Ok(crate::runtime::Value::Str(a + &other.to_string())),
            (other, crate::runtime::Value::Str(b)) => return Ok(crate::runtime::Value::Str(other.to_string() + &b)),
            (crate::runtime::Value::Int(a), crate::runtime::Value::Int(b)) => return Ok(crate::runtime::Value::Int(a + b)),
            (crate::runtime::Value::Float(a), crate::runtime::Value::Float(b)) => return Ok(crate::runtime::Value::Float(a + b)),
            (crate::runtime::Value::Int(a), crate::runtime::Value::Float(b)) => return Ok(crate::runtime::Value::Float(a as f64 + b)),
            (crate::runtime::Value::Float(a), crate::runtime::Value::Int(b)) => return Ok(crate::runtime::Value::Float(a + b as f64)),
            (a, b) => return Ok(crate::runtime::Value::Str(a.to_string() + &b.to_string())),
        }
    }

    // binary - handling (numeric subtraction)
    if let Some((left, right)) = split_top_level(s, '-') {
        // Make sure it's not a negative number (check if left side is empty or operator)
        if !left.is_empty() && !left.ends_with('(') {
            let lv = eval_expr(left, env)?;
            let rv = eval_expr(right, env)?;
            match (lv, rv) {
                (crate::runtime::Value::Int(a), crate::runtime::Value::Int(b)) => return Ok(crate::runtime::Value::Int(a - b)),
                (crate::runtime::Value::Float(a), crate::runtime::Value::Float(b)) => return Ok(crate::runtime::Value::Float(a - b)),
                (crate::runtime::Value::Int(a), crate::runtime::Value::Float(b)) => return Ok(crate::runtime::Value::Float(a as f64 - b)),
                (crate::runtime::Value::Float(a), crate::runtime::Value::Int(b)) => return Ok(crate::runtime::Value::Float(a - b as f64)),
                _ => {}
            }
        }
    }

    // binary * handling (numeric multiplication)
    if let Some((left, right)) = split_top_level(s, '*') {
        let lv = eval_expr(left, env)?;
        let rv = eval_expr(right, env)?;
        match (lv, rv) {
            (crate::runtime::Value::Int(a), crate::runtime::Value::Int(b)) => return Ok(crate::runtime::Value::Int(a * b)),
            (crate::runtime::Value::Float(a), crate::runtime::Value::Float(b)) => return Ok(crate::runtime::Value::Float(a * b)),
            (crate::runtime::Value::Int(a), crate::runtime::Value::Float(b)) => return Ok(crate::runtime::Value::Float(a as f64 * b)),
            (crate::runtime::Value::Float(a), crate::runtime::Value::Int(b)) => return Ok(crate::runtime::Value::Float(a * b as f64)),
            _ => {}
        }
    }

    // binary / handling (numeric division)
    if let Some((left, right)) = split_top_level(s, '/') {
        let lv = eval_expr(left, env)?;
        let rv = eval_expr(right, env)?;
        match (lv, rv) {
            (crate::runtime::Value::Int(a), crate::runtime::Value::Int(b)) => {
                if b == 0 {
                    return Err(anyhow!("Division by zero"));
                }
                return Ok(crate::runtime::Value::Int(a / b));
            }
            (crate::runtime::Value::Float(a), crate::runtime::Value::Float(b)) => {
                if b == 0.0 {
                    return Err(anyhow!("Division by zero"));
                }
                return Ok(crate::runtime::Value::Float(a / b));
            }
            (crate::runtime::Value::Int(a), crate::runtime::Value::Float(b)) => {
                if b == 0.0 {
                    return Err(anyhow!("Division by zero"));
                }
                return Ok(crate::runtime::Value::Float(a as f64 / b));
            }
            (crate::runtime::Value::Float(a), crate::runtime::Value::Int(b)) => {
                if b == 0 {
                    return Err(anyhow!("Division by zero"));
                }
                return Ok(crate::runtime::Value::Float(a / b as f64));
            }
            _ => {}
        }
    }
    // literals
    if s.starts_with('"') && s.ends_with('"') {
        return Ok(crate::runtime::Value::Str(s[1..s.len() - 1].to_string()));
    }
    if s == "True" || s == "true" {
        return Ok(crate::runtime::Value::Bool(true));
    }
    if s == "False" || s == "false" {
        return Ok(crate::runtime::Value::Bool(false));
    }
    if let Ok(i) = s.parse::<i64>() {
        return Ok(crate::runtime::Value::Int(i));
    }
    if let Ok(f) = s.parse::<f64>() {
        return Ok(crate::runtime::Value::Float(f));
    }

    // attribute access: obj.attr (no call)
    if s.contains('.') && !s.contains('(') {
        if let Some((obj, attr)) = s.rsplit_once('.') {
            let obj = obj.trim();
            let attr = attr.trim();
            if let Some(v) = env.get(obj) {
                match v {
                    crate::runtime::Value::Instance(_, fields, _) => {
                        if let Some(fv) = fields.get(attr) {
                            return Ok(fv.clone());
                        }
                    }
                    crate::runtime::Value::Dict(map) => {
                        if let Some(fv) = map.get(attr) {
                            return Ok(fv.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // function call like name(arg1, arg2) or method call obj.method(args)
    if let Some((fname, _rest)) = s.split_once('(') {
        if s.ends_with(')') {
            let fname = fname.trim();
            let args = &s[fname.len() + 1..s.len() - 1];
            let args_vec: Vec<crate::runtime::Value> = if args.trim().is_empty() {
                vec![]
            } else {
                args.split(',')
                    .map(|a| eval_expr(a, env))
                    .collect::<Result<Vec<_>>>()?
            };

            // method call: obj.method(...)
            if let Some((obj_name, method_name)) = fname.rsplit_once('.') {
                let obj_name = obj_name.trim();
                let method_name = method_name.trim();
                if let Some(obj_val) = env.get(obj_name) {
                    match obj_val.clone() {
                        crate::runtime::Value::Instance(_class_name, _fields, methods) => {
                            if let Some(func) = methods.get(method_name) {
                                match func {
                                    crate::runtime::Function::Native(nf) => {
                                        // For native methods, prepend self as the first argument
                                        let mut method_args = vec![obj_val.clone()];
                                        method_args.extend(args_vec);
                                        return nf(method_args);
                                    }
                                    crate::runtime::Function::User { params, body } => {
                                        // create child env, bind self and params
                                        let mut child = Env::new();
                                        child.set("self".to_string(), obj_val.clone());
                                        for (i, p) in params.iter().enumerate() {
                                            if p == "self" { continue; }
                                            if let Some(a) = args_vec.get(i) {
                                                child.set(p.clone(), a.clone());
                                            } else {
                                                child.set(p.clone(), crate::runtime::Value::None);
                                            }
                                        }
                                        crate::eval::run_stmts(body, &mut child)?;
                                        return Ok(crate::runtime::Value::None);
                                    }
                                }
                            } else {
                                return Err(anyhow!("Method not found: {}", method_name));
                            }
                        }
                        crate::runtime::Value::Class(_class_name, methods) => {
                            // calling class method statically or constructing? try methods
                            if let Some(func) = methods.get(method_name) {
                                match func {
                                    crate::runtime::Function::Native(nf) => return nf(args_vec),
                                    crate::runtime::Function::User { params, body } => {
                                        let mut child = Env::new();
                                        for (i, p) in params.iter().enumerate() {
                                            if let Some(a) = args_vec.get(i) {
                                                child.set(p.clone(), a.clone());
                                            }
                                        }
                                        crate::eval::run_stmts(body, &mut child)?;
                                        return Ok(crate::runtime::Value::None);
                                    }
                                }
                            }
                        }
                        crate::runtime::Value::Dict(map) => {
                            // allow module-like dicts to expose functions by name (string ref)
                            if let Some(val) = map.get(method_name) {
                                match val.clone() {
                                    crate::runtime::Value::Str(ref fname) => {
                                        // call the global function with that name
                                        return env.call(&fname, args_vec);
                                    }
                                    other => {
                                        // if it's a stored callable-like instance, try to call via env.call
                                        if let crate::runtime::Value::Str(fname) = other {
                                            return env.call(&fname, args_vec);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                return Err(anyhow!("Undefined method call: {}", fname));
            }

            // regular function
            return env.call(fname, args_vec);
        }
    }

    // identifier
    if let Some(v) = env.get(s) {
        return Ok(v.clone());
    }

    Err(anyhow!("Unknown expression: {}", s))
}
