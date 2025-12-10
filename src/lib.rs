//! # Ferrum Language Interpreter
//!
//! Ferrum is a small yet powerful interpreted language written in Rust, featuring:
//! - Full object-oriented programming support with classes and inheritance
//! - Comprehensive standard library (math, I/O, networking, GUI, computer vision)
//! - Interactive REPL with direct expression evaluation
//! - GUI development with Iced framework integration
//! - Computer vision capabilities via OpenCV bindings
//! - Code analysis and static checking
//! 
//! ## Quick Start
//!
//! ```ignore
//! use ferrum::interpreter;
//!
//! // Run a Ferrum script
//! interpreter::run_file("script.fm")?;
//!
//! // Start interactive REPL
//! interpreter::repl()?;
//!
//! // Compile to bytecode
//! interpreter::build("script.fm")?;
//! ```
//!
//! ## Example Script
//!
//! ```ferrum
//! x = 5
//! y = 10
//! print(x + y)  # Output: 15
//! 
//! class Person:
//!     def __init__(self, name):
//!         self.name = name
//!     def greet(self):
//!         print("Hello, " + self.name)
//!
//! p = Person("Alice")
//! p.greet()  # Output: Hello, Alice
//! ```

pub mod parser;
pub mod eval;
pub mod runtime;
pub mod repl;
pub mod stdlib;
pub mod ui;

/// High-level interpreter interface for running Ferrum scripts
pub mod interpreter {
    use anyhow::Result;
    use std::path::Path;

    /// Run a Ferrum script from a file
    /// 
    /// # Arguments
    /// * `path` - Path to the `.fm` script file
    /// 
    /// # Returns
    /// Ok(()) on successful execution, or an error message
    /// 
    /// # Example
    /// ```ignore
    /// ferrum::interpreter::run_file("example.fm")?;
    /// ```
    pub fn run_file(path: &str) -> Result<()> {
        let path = Path::new(path);
        let src = std::fs::read_to_string(path)?;
        let mut env = crate::runtime::Env::new();
        crate::stdlib::register_stdlib(&mut env);
        crate::parser::parse_and_run(&src, &mut env)?;
        Ok(())
    }

    /// Start the interactive REPL (Read-Eval-Print Loop)
    /// 
    /// # Returns
    /// Ok(()) when user exits REPL, or an error
    /// 
    /// # Example
    /// ```ignore
    /// ferrum::interpreter::repl()?;
    /// ```
    pub fn repl() -> Result<()> {
        crate::repl::start()
    }

    /// Build a Ferrum script to bytecode (AST serialization)
    /// 
    /// This is a prototype implementation that serializes the parsed AST
    /// to a `.fmb` file for future compilation or distribution.
    /// 
    /// # Arguments
    /// * `path` - Path to the `.fm` script file
    /// 
    /// # Returns
    /// Ok(()) on successful build, or an error
    /// 
    /// # Example
    /// ```ignore
    /// ferrum::interpreter::build("example.fm")?;
    /// // Creates example.fm.fmb
    /// ```
    pub fn build(path: &str) -> Result<()> {
        // Prototype: read file and write AST debug to a .fmb file
        let src = std::fs::read_to_string(path)?;
        let ast = crate::parser::parse_to_ast(&src)?;
        let out = format!("{}.fmb", path);
        let s = format!("{:?}", ast);
        std::fs::write(&out, s.as_bytes())?;
        println!("Built {} -> {} (AST debug)", path, out);
        Ok(())
    }
}

