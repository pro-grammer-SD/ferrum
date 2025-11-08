use std::fs;
use std::process::Command;
use std::path::Path;

mod transpiler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Check for help flag
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help" || args[1] == "help") {
        print_help();
        return;
    }
    
    println!("🔥 Ferrum Compiler v1.0");
    
    if args.len() < 2 {
        eprintln!("❌ Error: No input file specified\n");
        print_usage();
        std::process::exit(1);
    }
    
    let fm_file = &args[1];
    
    if !Path::new(fm_file).exists() {
        eprintln!("❌ Error: File '{}' not found", fm_file);
        std::process::exit(1);
    }

    let exe_file = fm_file.replace(".fm", ".exe");

    println!("📖 Reading: {}", fm_file);
    let ferrum_code = match fs::read_to_string(fm_file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("🔄 Transpiling to Rust...");
    let rust_code = transpiler::transpile_to_rust(&ferrum_code);
    
    // Use temp directory for intermediate file
    let temp_dir = std::env::temp_dir();
    let temp_rs = temp_dir.join(format!("ferrum_{}.rs", std::process::id()));
    
    match fs::write(&temp_rs, &rust_code) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("❌ Failed to write temporary file: {}", e);
            std::process::exit(1);
        }
    }

    println!("✅ Transpiled successfully");
    println!("🔨 Compiling with rustc...");

    let status = Command::new("rustc")
        .args([
            temp_rs.to_str().unwrap(),
            "-O",
            "-o",
            &exe_file
        ])
        .status();

    // Clean up temp file
    let _ = fs::remove_file(&temp_rs);

    match status {
        Ok(s) if s.success() => {
            println!("✅ Built successfully → {}", exe_file);
            println!();
            let pdb_file = exe_file.replace(".exe", ".pdb");
            if Path::new(&pdb_file).exists() {
                let _ = fs::remove_file(&pdb_file);
            }
            println!("🚀 Running {}...", exe_file);
            println!("{}", "=".repeat(50));
            
            // Run the compiled executable
            let run_status = Command::new(&exe_file)
                .status();
            
            println!("{}", "=".repeat(50));
            
            match run_status {
                Ok(s) if s.success() => {
                    println!("✅ Program executed successfully");
                }
                Ok(_) => {
                    eprintln!("⚠️  Program exited with error");
                }
                Err(e) => {
                    eprintln!("❌ Failed to run program: {}", e);
                }
            }
        }
        Ok(_) => {
            eprintln!("❌ Compilation failed");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ Failed to run rustc: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("🔥 Ferrum Compiler v1.0");
    println!();
    println!("DESCRIPTION:");
    println!("    Ferrum is a fast, Python-easy language that compiles to native executables");
    println!("    via Rust. Write simple .fm files and get blazing-fast .exe binaries!");
    println!();
    println!("USAGE:");
    println!("    ferrum_compiler <file.fm>");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help, help    Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    ferrum_compiler program.fm       Compile and run program.fm");
    println!("    ferrum_compiler examples/demo.fm  Compile and run demo");
    println!();
    println!("LANGUAGE FEATURES:");
    println!("    • Variables with types:  x:int = 42");
    println!("    • Input from user:       x:int = input(\"Enter number\")");
    println!("    • Type casting:          y:float = type_cast(\"3.14\", float)");
    println!("    • Lists:                 z:list = 1, 2, \"hello\"");
    println!("    • Functions:             fn add(a:int, b:int) -> int:");
    println!("    • If/else blocks:        if x > 10: ... else: ...");
    println!("    • Print output:          print x, y, \"text\"");
    println!("    • Comments:              ! This is a comment");
    println!();
    println!("SUPPORTED TYPES:");
    println!("    int, float, str, bool, list");
    println!();
    println!("WEBSITE:");
    println!("    https://github.com/pro-grammer-SD/ferrum");
}

fn print_usage() {
    println!("USAGE:");
    println!("    ferrum_compiler <file.fm>");
    println!();
    println!("For more information, run: ferrum_compiler --help");
}
