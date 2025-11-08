# 🔥 Ferrum Programming Language

**Python-easy syntax. Rust-level performance. Native executables.**

Ferrum is a blazingly fast compiled programming language that combines the simplicity of Python with the performance of Rust. Write `.fm` files with clean, intuitive syntax and compile them directly to native `.exe` binaries.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

## ✨ Features

- 🚀 **Blazing Fast** - Compiles to native machine code via Rust
- 🐍 **Python-like Syntax** - Easy to learn and write
- ⚡ **Zero Runtime Overhead** - No interpreter, no VM
- 📦 **Standalone Executables** - Single `.exe` file, no dependencies
- 🎯 **Type Safety** - Static typing with type inference
- 🔧 **Simple Tooling** - One command to compile and run

## 🚀 Quick Start

### Installation

1. **Prerequisites**: Install Rust from [rustup.rs](https://rustup.rs/)

2. **Build Ferrum Compiler**:
```bash
git clone https://github.com/pro-grammer-SD/ferrum.git
cd ferrum
cargo build --release
```

3. **Add to PATH** (optional):
```bash
# Windows: Copy dist/ferrum_compiler.exe to a folder in your PATH
copy dist\ferrum_compiler.exe C:\path\to\bin\
```

### Hello World

Create `hello.fm`:
```ferrum
! My first Ferrum program
print "Hello, World!";
```

Compile and run:
```bash
ferrum_compiler hello.fm
```

Output:
```
🔥 Ferrum Compiler v1.0
📖 Reading: hello.fm
🔄 Transpiling to Rust...
✅ Transpiled successfully
🔨 Compiling with rustc...
✅ Built successfully → hello.exe

🚀 Running hello.exe...
==================================================
Hello, World!
==================================================
✅ Program executed successfully
```

## 📚 Language Guide

### Variables & Types

```ferrum
! Variable declarations with types
x:int = 42;
y:float = 3.14;
name:str = "Ferrum";
active:bool = true;

! Lists (dynamic arrays)
numbers:list = 1, 2, 3, 4, 5;
mixed:list = 42, "hello", 3.14;
```

### User Input

```ferrum
! Get input from user
age:int = input("Enter your age");
name:str = input("Enter your name");

print "Hello", name, "you are", age, "years old";
```

### Type Casting

```ferrum
! Convert string to other types
pi:float = type_cast("3.14159", float);
count:int = type_cast("100", int);
```

### Functions

```ferrum
! Function with parameters and return type
fn add(a:int, b:int) -> int:
    return a + b;

fn greet(name:str) -> str:
    return "Hello, " + name;

! Call functions
result:int = add(10, 20);
message:str = greet("World");
```

### Control Flow

```ferrum
! If-else statements
score:int = input("Enter score");

if score >= 90:
    print "Grade: A";
else:
    print "Grade: B or lower";

! Comparison operators: >, <, >=, <=, ==, !=
if x > 10:
    print "x is greater than 10";
```

### Printing Output

```ferrum
! Print single value
print "Hello";
print 42;

! Print multiple values
x:int = 10;
y:int = 20;
print "x:", x, "y:", y;

! Mix strings and variables
print "The answer is", 42, "!";
```

### Comments

```ferrum
! This is a single-line comment
! Comments start with exclamation mark

x:int = 42;  ! Inline comments work too
```

## 🎯 Complete Example

Create `calculator.fm`:
```ferrum
! Simple Calculator
print "=== Ferrum Calculator ===";

a:int = input("Enter first number");
b:int = input("Enter second number");

fn add(x:int, y:int) -> int:
    return x + y;

fn multiply(x:int, y:int) -> int:
    return x * y;

sum:int = add(a, b);
product:int = multiply(a, b);

print "Sum:", sum;
print "Product:", product;

if sum > 100:
    print "That's a big sum!";
else:
    print "That's a normal sum.";
```

Compile and run:
```bash
ferrum_compiler calculator.fm
```

## 📖 Supported Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | 32-bit integer | `42`, `-10`, `1000` |
| `float` | 64-bit floating point | `3.14`, `-0.5`, `2.0` |
| `str` | String (UTF-8) | `"hello"`, `"world"` |
| `bool` | Boolean | `true`, `false` |
| `list` | Dynamic array | `1, 2, 3`, `"a", "b"` |

## 🛠️ Compiler Commands

```bash
# Compile and run a Ferrum file
ferrum_compiler program.fm

# Show help
ferrum_compiler --help
ferrum_compiler -h
ferrum_compiler help
```

## 🔧 How It Works

1. **Transpilation**: Ferrum code (`.fm`) is transpiled to Rust code
2. **Compilation**: Rust code is compiled to native machine code using `rustc`
3. **Optimization**: Full optimizations enabled (`-O` flag)
4. **Execution**: The compiled binary runs immediately
5. **Cleanup**: Temporary files are automatically deleted

```
.fm file → Transpiler → .rs file → rustc → .exe file → Run!
```

## 🚀 Why Ferrum?

### vs Python
- ⚡ **100x faster** - Compiled to native code, no interpreter
- 📦 **Single executable** - No need to install Python runtime
- 🎯 **Type safety** - Catch errors at compile time

### vs Rust
- 🐍 **Easier syntax** - Python-like, no complex lifetime management
- 📝 **Less boilerplate** - Simple variable declarations
- 🚀 **Faster development** - Write code quickly, compile once

### vs C/C++
- 🛡️ **Memory safe** - Compiles through Rust's safety checks
- 📚 **Simpler syntax** - No pointers, no manual memory management
- 🔧 **Modern tooling** - Easy setup, single command compilation

## 📁 Project Structure

```
ferrum/
├── src/
│   ├── main.rs          # Compiler entry point
│   └── transpiler.rs    # Ferrum → Rust transpiler
├── examples/
│   └── demo.fm          # Example programs
├── dist/
│   └── ferrum_compiler.exe  # Compiled compiler
├── Cargo.toml           # Rust dependencies
├── build.bat            # Build script
└── README.md            # This file
```

## 🎓 Examples

Check out the `examples/` directory for more:
- `demo.fm` - Language features showcase
- `hello.fm` - Hello World
- `calculator.fm` - Simple calculator
- `fibonacci.fm` - Fibonacci sequence
- `guess.fm` - Number guessing game

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. 🐛 **Report bugs** - Open an issue
2. 💡 **Suggest features** - Share your ideas
3. 🔧 **Submit PRs** - Improve the compiler or add examples
4. 📚 **Write docs** - Help others learn Ferrum

## 📜 License

MIT License - feel free to use Ferrum for any project!

## 🗺️ Roadmap

- [ ] For loops and while loops
- [ ] String operations and methods
- [ ] File I/O support
- [ ] Standard library functions
- [ ] Better error messages
- [ ] Syntax highlighting for editors
- [ ] Package manager
- [ ] Cross-platform compilation

## 💬 Community

- 🐙 **GitHub**: [github.com/pro-grammer-SD/ferrum](https://github.com/pro-grammer-SD/ferrum)
- 💡 **Issues**: Report bugs and request features
- ⭐ **Star**: If you like Ferrum, give it a star!

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) - The backbone of Ferrum
- Inspired by Python's simplicity and Rust's performance

---

**Made with 🔥 by passionate developers who love fast code and clean syntax**

*Ferrum: Where Python meets Performance*
