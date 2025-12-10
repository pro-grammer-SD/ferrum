use anyhow::Result;
use std::io::{self, Write};

/// Start the Ferrum interactive REPL (Read-Eval-Print Loop)
/// 
/// Features:
/// - Interactive statement and expression evaluation
/// - Direct expression evaluation with >>> prefix returns values
/// - Support for all Ferrum language features
/// - Type casting, list operations, class definitions
/// - Module/library access via global stdlib
/// 
/// Examples:
/// >>> 1 + 2
/// 3
/// >>> print("Hello")
/// Hello
/// >>> x = 5
/// >>> x * 2
/// 10
pub fn start() -> Result<()> {
    println!("Ferrum REPL — prototype. Type 'exit' to quit.");
    println!("Use '>>>' prefix to get expression values directly, or 'print()' for output.");
    let mut env = crate::runtime::Env::new();
    crate::stdlib::register_stdlib(&mut env);
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let l = line.trim();
        if l == "exit" || l == "quit" {
            break;
        }
        if l.is_empty() { continue; }
        
        // Check if user is using direct >>> expression evaluation
        let (is_expr_mode, code) = if l.starts_with(">>>") {
            (true, l[3..].trim().to_string())
        } else {
            (false, l.to_string())
        };
        
        // Try to parse as statement
        let stmt = crate::parser::parse_to_ast(&code);
        match stmt {
            Ok(ast) => {
                if is_expr_mode {
                    // In expression mode, try to evaluate and print the result
                    match crate::eval::eval_expr_str(&code, &mut env) {
                        Ok(value) => {
                            println!("{}", value.to_string());
                        }
                        Err(_e) => {
                            // Fall back to statement execution
                            let _ = crate::eval::run_stmts(&ast.body, &mut env);
                        }
                    }
                } else {
                    // Regular statement execution
                    let _ = crate::eval::run_stmts(&ast.body, &mut env);
                }
            }
            Err(e) => println!("Parse error: {}", e),
        }
    }
    Ok(())
}

