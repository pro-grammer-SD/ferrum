pub fn transpile_to_rust(ferrum_code: &str) -> String {
    let mut rust_code = String::from("use std::io::{self, Write};\n\n");
    let lines: Vec<&str> = ferrum_code.lines().collect();
    // Separate functions from main code
    let (main_lines, functions) = separate_functions(&lines);
    rust_code.push_str("fn main() {\n");
    rust_code.push_str(&transpile_block(&main_lines, 1));
    rust_code.push_str("}\n\n");
    // Add function definitions
    for func_lines in functions {
        rust_code.push_str(&transpile_function(&func_lines));
        rust_code.push_str("\n\n");
    }
    rust_code
}

// ... Helper extraction, unchanged ...

fn separate_functions<'a>(lines: &'a [&'a str]) -> (Vec<&'a str>, Vec<Vec<&'a str>>) {
    let mut main_lines = Vec::new();
    let mut functions = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with("fn ") {
            // Collect function lines
            let mut func_lines = vec![lines[i]];
            i += 1;
            // Get function body until return statement
            while i < lines.len() {
                let body_line = lines[i].trim();
                if body_line.is_empty() || body_line.starts_with('!') {
                    i += 1;
                    continue;
                }
                func_lines.push(lines[i]);
                if body_line.starts_with("return ") {
                    i += 1;
                    break;
                }
                // Stop if we hit another function or top-level var
                if body_line.starts_with("fn ")
                    || (body_line.contains(":int =")
                        || body_line.contains(":float =")
                        || body_line.contains(":str =")
                        || body_line.contains(":list ="))
                {
                    break;
                }
                i += 1;
            }
            functions.push(func_lines);
        } else {
            main_lines.push(lines[i]);
            i += 1;
        }
    }
    (main_lines, functions)
}

fn transpile_block(lines: &[&str], indent_level: usize) -> String {
    let mut code = String::new();
    let indent = "    ".repeat(indent_level);
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        // Skip empty and comments
        if line.is_empty() || line.starts_with('!') {
            i += 1;
            continue;
        }
        // Handle if-else blocks
        if line.starts_with("if ") {
            let (if_block, consumed) = transpile_if_block(&lines[i..], indent_level);
            code.push_str(&if_block);
            i += consumed;
            continue;
        }
        // Regular statement
        code.push_str(&indent);
        code.push_str(&transpile_statement(line));
        code.push('\n');
        i += 1;
    }
    code
}

fn transpile_if_block(lines: &[&str], indent_level: usize) -> (String, usize) {
    let mut code = String::new();
    let indent = "    ".repeat(indent_level);
    let mut i = 0;
    // Parse if condition
    let if_line = lines[i].trim();
    let condition = if_line.strip_prefix("if ")
        .unwrap_or(if_line)
        .trim_end_matches(':');
    code.push_str(&indent);
    code.push_str(&format!("if {} {{\n", condition));
    i += 1;
    // Collect if body until else or end
    let mut if_body = Vec::new();
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || line.starts_with('!') {
            i += 1;
            continue;
        }
        if line == "else:" {
            break;
        }
        // Stop at next top-level statement
        if line.starts_with("if ")
            || line.starts_with("fn ")
            || (line.contains(':') && line.contains('=') && !line.starts_with("    ")
                && (line.contains(":int")
                    || line.contains(":float")
                    || line.contains(":str")
                    || line.contains(":list")))
        {
            break;
        }
        if_body.push(lines[i]);
        i += 1;
    }
    // Transpile if body
    for body_line in if_body {
        let trimmed = body_line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('!') {
            code.push_str(&"    ".repeat(indent_level + 1));
            code.push_str(&transpile_statement(trimmed));
            code.push('\n');
        }
    }
    code.push_str(&indent);
    code.push_str("}");
    // Check for else
    if i < lines.len() && lines[i].trim() == "else:" {
        code.push_str(" else {\n");
        i += 1;
        // Collect else body
        let mut else_body = Vec::new();
        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with('!') {
                i += 1;
                continue;
            }
            // Stop at next top-level statement
            if line.starts_with("if ")
                || line.starts_with("fn ")
                || (line.contains(':')
                    && line.contains('=')
                    && (line.contains(":int")
                        || line.contains(":float")
                        || line.contains(":str")
                        || line.contains(":list")))
            {
                break;
            }
            else_body.push(lines[i]);
            i += 1;
        }
        // Transpile else body
        for body_line in else_body {
            let trimmed = body_line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('!') {
                code.push_str(&"    ".repeat(indent_level + 1));
                code.push_str(&transpile_statement(trimmed));
                code.push('\n');
            }
        }
        code.push_str(&indent);
        code.push_str("}");
    }
    code.push('\n');
    (code, i)
}

fn transpile_function(lines: &[&str]) -> String {
    let header = lines[0].trim();
    let mut code = String::new();
    // Parse function header
    let (fn_signature, return_type) = parse_function_header(header);
    code.push_str(&format!("fn {} -> {} {{\n", fn_signature, return_type));
    // Transpile body
    for i in 1..lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || line.starts_with('!') {
            continue;
        }
        code.push_str("    ");
        code.push_str(&transpile_statement(line));
        code.push('\n');
    }
    code.push_str("}\n");
    code
}

fn parse_function_header(header: &str) -> (String, String) {
    let header = header.strip_prefix("fn ").unwrap_or(header).trim_end_matches(':');
    let parts: Vec<&str> = header.split("->").collect();
    let sig_part = parts[0].trim();
    let return_type = if parts.len() > 1 {
        convert_type(parts[1].trim())
    } else {
        "()"
    };
    let paren_pos = sig_part.find('(').unwrap_or(sig_part.len());
    let fn_name = &sig_part[..paren_pos];
    let params = if paren_pos < sig_part.len() {
        convert_params(&sig_part[paren_pos..])
    } else {
        "()".to_string()
    };
    (format!("{}{}", fn_name, params), return_type.to_string())
}

fn transpile_statement(line: &str) -> String {
    let line = line.trim();
    // Variable declaration
    if line.contains(':') && line.contains('=') && !line.starts_with("if ") {
        return transpile_var_decl(line);
    }
    // Print
    if line.starts_with("print ") {
        return transpile_print(line);
    }
    // Return
    if line.starts_with("return ") {
        let value = line.strip_prefix("return ").unwrap_or("").trim_end_matches(';');
        return format!("return {};", value);
    }
    line.to_string()
}

fn transpile_var_decl(line: &str) -> String {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return line.to_string();
    }
    let left = parts[0].trim();
    let right = parts[1].trim().trim_end_matches(';');
    let var_parts: Vec<&str> = left.split(':').collect();
    if var_parts.len() != 2 {
        return line.to_string();
    }
    let var_name = var_parts[0].trim();
    let var_type = var_parts[1].trim();
    // Handle input
    if right.starts_with("input(") {
        return transpile_input(var_name, var_type, right);
    }
    // Handle type_cast
    if right.starts_with("type_cast(") {
        return transpile_type_cast(var_name, var_type, right);
    }
    // Handle list
    if var_type == "list" {
        let items: Vec<&str> = right.split(',').map(|s| s.trim()).collect();
        let values = items.iter().map(|item| {
            if item.starts_with('"') && item.ends_with('"') {
                format!("{}.to_string()", item)
            } else {
                format!("{}.to_string()", item)
            }
        }).collect::<Vec<_>>().join(", ");
        return format!("let {} = vec![{}];", var_name, values);
    }
    let rust_type = convert_type(var_type);
    format!("let {}: {} = {};", var_name, rust_type, right)
}

fn transpile_input(var_name: &str, var_type: &str, input_call: &str) -> String {
    let prompt = extract_string(input_call).unwrap_or("Enter value");
    let rust_type = convert_type(var_type);
    let mut code = format!("let {};\n", var_name);
    code.push_str("    {\n");
    code.push_str(&format!("        print!(\"{}: \");\n", prompt));
    code.push_str("        io::stdout().flush().unwrap();\n");
    code.push_str("        let mut input_line = String::new();\n");
    code.push_str("        io::stdin().read_line(&mut input_line).unwrap();\n");
    code.push_str(&format!("        {} = ", var_name));
    match rust_type {
        "i32" | "i64" | "u32" | "u64" => {
            code.push_str("input_line.trim().parse().unwrap_or(0);\n");
        }
        "f64" | "f32" => {
            code.push_str("input_line.trim().parse().unwrap_or(0.0);\n");
        }
        _ => {
            code.push_str("input_line.trim().to_string();\n");
        }
    }
    code.push_str("    }");
    code
}

// FIXED! Robust type_cast for variables and literals!
fn transpile_type_cast(var_name: &str, var_type: &str, cast_call: &str) -> String {
    // Example: type_cast(a, float)
    // Parse type_cast(val, typ)
    let raw_args = cast_call.trim_start_matches("type_cast(").trim_end_matches(")");
    let parts: Vec<&str> = raw_args.split(',').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        // Fallback: just assign 0, or "0"
        let rust_type = convert_type(var_type);
        return match rust_type {
            "f64" | "f32" => format!("let {}: {} = 0.0;", var_name, rust_type),
            "i32" | "i64" => format!("let {}: {} = 0;", var_name, rust_type),
            _ => format!("let {}: {} = \"0\".to_string();", var_name, rust_type)
        };
    }
    let value = parts[0];
    let _target = parts[1];
    let rust_type = convert_type(var_type);

    // Don't wrap in quotes unless it's a literal string
    let value_expr = if value.starts_with('"') && value.ends_with('"') { value.to_string() } else { format!("{}", value) };

    match rust_type {
        "f64" | "f32" => format!("let {}: {} = {}.to_string().parse().unwrap_or(0.0);", var_name, rust_type, value_expr),
        "i32" | "i64" => format!("let {}: {} = {}.to_string().parse().unwrap_or(0);", var_name, rust_type, value_expr),
        "String" => format!("let {}: String = {}.to_string();", var_name, value_expr),
        _ => format!("let {}: {} = {};", var_name, rust_type, value_expr)
    }
}

fn transpile_print(line: &str) -> String {
    let content = line.strip_prefix("print ").unwrap_or("").trim_end_matches(';');
    if content.contains(',') {
        let items: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        let mut format_parts = Vec::new();
        let mut args = Vec::new();
        for item in items {
            if item.starts_with('"') && item.ends_with('"') {
                format_parts.push(item.trim_matches('"').to_string());
            } else {
                format_parts.push(format!("{}: {{:?}}", item));
                args.push(item.to_string());
            }
        }
        let format_str = format_parts.join(", ");
        if args.is_empty() {
            return format!("println!(\"{}\");", format_str);
        }
        return format!("println!(\"{}\", {});", format_str, args.join(", "));
    }
    if content.starts_with('"') && content.ends_with('"') {
        format!("println!({});", content)
    } else {
        format!("println!(\"{{:?}}\", {});", content)
    }
}

fn convert_type(ferrum_type: &str) -> &str {
    match ferrum_type.trim() {
        "int" => "i32",
        "float" => "f64",
        "str" | "string" => "String",
        "bool" => "bool",
        t => t,
    }
}

fn convert_params(params: &str) -> String {
    params
        .replace(":int", ": i32")
        .replace(":float", ": f64")
        .replace(":str", ": &str")
        .replace(":bool", ": bool")
}

fn extract_string(s: &str) -> Option<&str> {
    let start = s.find('"')?;
    let end = s[start + 1..].find('"')?;
    Some(&s[start + 1..start + 1 + end])
}
