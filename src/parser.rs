use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::eval::Stmt;

/// Parsed module representation
/// 
/// The ModuleAST is the root node of a parsed Ferrum script's abstract syntax tree.
/// It contains a sequence of statements to be executed in order.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleAST {
    pub body: Vec<Stmt>,
}

/// Very small indentation-based parser that turns source into nested statements.
/// 
/// This is a compact prototype parser and supports a subset of Ferrum syntax:
/// - Class and function definitions
/// - Control flow (if/elif/else, while, for)
/// - Variable assignments and expressions
/// - Print statements
/// - Comments (lines starting with #)
/// 
/// # Example
/// ```ignore
/// let source = "x = 5\nprint(x)";
/// let ast = parse_to_ast(source)?;
/// ```
pub fn parse_to_ast(src: &str) -> Result<ModuleAST> {
    let mut lines: Vec<(usize, String)> = Vec::new();
    for raw in src.lines() {
        let s = raw.to_string();
        if s.trim().is_empty() {
            continue;
        }
        let indent = s.chars().take_while(|c| *c == ' ').count();
        let trimmed = s[indent..].to_string();
        lines.push((indent, trimmed));
    }

    let mut q: VecDeque<(usize, String)> = VecDeque::from(lines);
    let body = parse_block(0, &mut q)?;
    Ok(ModuleAST { body })
}

fn parse_block(min_indent: usize, q: &mut VecDeque<(usize, String)>) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while let Some((indent, _line)) = q.front() {
        if *indent < min_indent {
            break;
        }
        let (indent, line) = q.pop_front().unwrap();
        if line.trim_start().starts_with("#") {
            continue;
        }
        if line.ends_with(":") {
            let header = line.trim_end_matches(':').to_string();
            let body = parse_block(indent + 1, q)?;
            let stmt = if header.starts_with("if ") || header.starts_with("elif ") {
                // We'll create an If with a single branch for simplicity
                let cond = header.splitn(2, ' ').nth(1).unwrap_or("true").to_string();
                Stmt::If(vec![(cond, body)], vec![])
            } else if header.starts_with("else") {
                Stmt::Else(body)
            } else if header.starts_with("while ") {
                let cond = header.splitn(2, ' ').nth(1).unwrap_or("true").to_string();
                Stmt::While(cond, body)
            } else if header.starts_with("for ") {
                // for x in expr:
                let rest = header.trim_start_matches("for ").to_string();
                Stmt::For(rest, body)
            } else if header.starts_with("def ") {
                // def name(args):
                let rest = header.trim_start_matches("def ").to_string();
                Stmt::Def(rest, body)
            } else if header.starts_with("class ") {
                let rest = header.trim_start_matches("class ").to_string();
                Stmt::Class(rest, body)
            } else {
                // Generic block (e.g., try:, except:)
                Stmt::Block(header, body)
            };
            stmts.push(stmt);
        } else {
            // Simple statement line
            let s = parse_simple_statement(line);
            stmts.push(s);
        }
    }
    Ok(stmts)
}

fn parse_simple_statement(line: String) -> Stmt {
    let s = line.trim().to_string();
    if s.starts_with("print(") && s.ends_with(")") {
        let inner = s[6..s.len() - 1].to_string();
        Stmt::Print(inner)
    } else if s.starts_with("return ") {
        let expr = s[7..].to_string();
        Stmt::Return(expr)
    } else if s.contains('=') {
        let parts: Vec<&str> = s.splitn(2, '=').collect();
        let lhs = parts[0].trim().to_string();
        let rhs = parts[1].trim().to_string();
        Stmt::Assign(lhs, rhs)
    } else if s.starts_with("import ") {
        let name = s.trim_start_matches("import ").trim().to_string();
        Stmt::Import(name)
    } else {
        Stmt::Expr(s)
    }
}

pub fn parse_and_run(src: &str, env: &mut crate::runtime::Env) -> Result<()> {
    let ast = parse_to_ast(src)?;
    crate::eval::run_stmts(&ast.body, env)
}
