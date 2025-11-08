pub fn transpile_to_rust(ferrum_code: &str) -> String {
    let mut rust_code = String::from("use std::io::{self, Write};\n\n");
    let raw_lines: Vec<&str> = ferrum_code.lines().collect();
    let (main_lines, functions) = separate_functions(&raw_lines);
    rust_code.push_str("fn main() {\n");
    rust_code.push_str(&transpile_block(&main_lines, 1));
    rust_code.push_str("}\n\n");
    for func_lines in functions {
        rust_code.push_str(&transpile_function(&func_lines));
        rust_code.push_str("\n\n");
    }
    rust_code
}

fn separate_functions<'a>(lines: &'a [&'a str]) -> (Vec<&'a str>, Vec<Vec<&'a str>>) {
    let mut main = Vec::new();
    let mut funcs = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        if trimmed.starts_with("fn ") {
            let base_indent = leading_indent(line);
            let mut block = vec![line];
            i += 1;
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() {
                    block.push(l);
                    i += 1;
                    continue;
                }
                let indent = leading_indent(l);
                if indent <= base_indent && !l.trim_start().starts_with('!') {
                    break;
                }
                block.push(l);
                i += 1;
            }
            funcs.push(block);
        } else {
            main.push(line);
            i += 1;
        }
    }
    (main, funcs)
}

fn transpile_block(lines: &[&str], indent_level: usize) -> String {
    let mut code = String::new();
    let indent = "    ".repeat(indent_level);
    let mut i = 0;
    while i < lines.len() {
        let raw = lines[i];
        let line = raw.trim_start();
        if line.is_empty() || line.starts_with('!') {
            i += 1;
            continue;
        }
        if line.starts_with("if ") {
            let (blk, consumed) = transpile_if_block(lines, i, indent_level);
            code.push_str(&blk);
            i = consumed;
            continue;
        }
        code.push_str(&indent);
        code.push_str(&transpile_statement(line));
        code.push('\n');
        i += 1;
    }
    code
}

fn transpile_if_block(lines: &[&str], start: usize, indent_level: usize) -> (String, usize) {
    let mut code = String::new();
    let mut i = start;
    let base_indent = leading_indent(lines[start]);
    let indent = "    ".repeat(indent_level);

    while i < lines.len() {
        let line = lines[i].trim_start();
        if line.starts_with("if ") || line.starts_with("elif ") {
            let condition = line.splitn(2, ' ').nth(1).unwrap_or("").trim_end_matches(':').trim();
            if line.starts_with("if ") {
                code.push_str(&format!("{}if {} {{\n", indent, normalize_expr(condition)));
            } else {
                code.push_str(&format!("{} else if {} {{\n", indent, normalize_expr(condition)));
            }

            i += 1;
            let mut body = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim_start().starts_with("elif ")
                    || l.trim_start().starts_with("else:")
                    || leading_indent(l) <= base_indent {
                    break;
                }
                body.push(l);
                i += 1;
            }
            code.push_str(&transpile_block(&body, indent_level + 1));
            code.push_str(&indent);
            code.push_str("}\n");
            continue;
        }

        if line.starts_with("else:") {
            code.push_str(&format!("{}else {{\n", indent));
            i += 1;
            let mut body = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if leading_indent(l) <= base_indent {
                    break;
                }
                body.push(l);
                i += 1;
            }
            code.push_str(&transpile_block(&body, indent_level + 1));
            code.push_str(&indent);
            code.push_str("}\n");
            continue;
        }
        break;
    }

    (code, i)
}

fn transpile_function(lines: &[&str]) -> String {
    let header = lines[0].trim_start();
    let (sig, ret) = parse_function_header(header);
    let mut code = format!("fn {} -> {} {{\n", sig, ret);
    for i in 1..lines.len() {
        let line = lines[i].trim();
        if line.is_empty() { continue; }
        code.push_str(&format!("    {}\n", transpile_statement(line)));
    }
    code.push_str("}\n");
    code
}

fn parse_function_header(header: &str) -> (String, String) {
    let header = header.strip_prefix("fn ").unwrap_or(header).trim_end_matches(':');
    let parts: Vec<&str> = header.split("->").collect();
    let sig = parts[0].trim();
    let ret = if parts.len() > 1 { convert_type(parts[1].trim()).to_string() } else { "()".to_string() };
    let name_end = sig.find('(').unwrap_or(sig.len());
    let name = &sig[..name_end];
    let params = if name_end < sig.len() { convert_params(&sig[name_end..]) } else { "()".to_string() };
    (format!("{}{}", name, params), ret)
}

fn transpile_statement(line: &str) -> String {
    let l = line.trim();
    if l.starts_with("input ") || l.starts_with("type_cast ") || l.starts_with("exit") || l.starts_with("print ") || l.starts_with("return ") {
        match l {
            l if l.starts_with("input ") => transpile_input_line(l),
            l if l.starts_with("type_cast ") => transpile_type_cast_line(l),
            l if l.starts_with("exit") => {
                let parts: Vec<&str> = l.split_whitespace().collect();
                let code = if parts.len() > 1 { parts[1] } else { "0" };
                format!("std::process::exit({});", code)
            },
            l if l.starts_with("print ") => transpile_print(l),
            l if l.starts_with("return ") => {
                let val = l.strip_prefix("return ").unwrap_or("").trim();
                format!("return {};", normalize_expr(val))
            },
            _ => l.to_string()
        }
    } else if l.contains('=') {
        let parts: Vec<&str> = l.splitn(2, '=').collect();
        let left = parts[0].trim();
        let right = normalize_expr(parts[1].trim());
        if left.contains(':') {
            let v: Vec<&str> = left.split(':').collect();
            let name = v[0].trim();
            let ty = convert_type(v[1].trim());
            format!("let {}: {} = {};", name, ty, right)
        } else {
            format!("{} = {};", left, right)
        }
    } else {
        normalize_expr(l)
    }
}

fn transpile_input_line(line: &str) -> String {
    let var = line.strip_prefix("input ").unwrap().trim();
    format!("let {} = {{ let mut input_line = String::new(); io::stdin().read_line(&mut input_line).unwrap(); input_line.trim().to_string() }};", var)
}

fn transpile_type_cast_line(line: &str) -> String {
    let content = line.strip_prefix("type_cast ").unwrap().trim();
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() != 2 { return line.to_string(); }
    let val = normalize_expr(parts[0]);
    let tgt = parts[1];
    match tgt {
        "int" => format!("{}.parse::<i32>().unwrap_or(0)", val),
        "float" => format!("{}.parse::<f64>().unwrap_or(0.0)", val),
        "str" | "string" => format!("{}.to_string()", val),
        _ => val,
    }
}

fn transpile_print(line: &str) -> String {
    let content = line.strip_prefix("print ").unwrap_or("").trim_end_matches(';').trim();

    let parts: Vec<&str> = content.split(',').map(|p| p.trim()).collect();

    let mut format_string = String::new();
    let mut args: Vec<String> = Vec::new();

    for part in parts {
        if part.starts_with('"') && part.ends_with('"') {
            // Append string literals without quotes to the format string
            format_string.push_str(&part[1..part.len() -1]);
        } else {
            format_string.push_str("{}");
            args.push(normalize_expr(part));
        }
        format_string.push(' ');
    }
    format_string = format_string.trim_end().to_string();

    if args.is_empty() {
        format!("println!(\"{}\");", format_string)
    } else {
        format!("println!(\"{}\", {});", format_string, args.join(", "))
    }
}

fn normalize_expr(s: &str) -> String {
    let s = s.trim();
    if s.is_empty() || (s.starts_with('"') && s.ends_with('"')) { return s.to_string(); }
    if s.contains('(') && s.contains(')') { return s.to_string(); }
    let operators = ["+", "-", "*", "/", "%", ">", "<", "==", ">=", "<=", "!="];
    if operators.iter().any(|op| s.contains(op)) { return s.to_string(); }
    let tokens: Vec<&str> = s.split_whitespace().collect();
    if tokens.len() >= 2 {
        let name = tokens[0];
        if !["print","if","else:","elif","while","for","fn","return","exit","type_cast","input"].contains(&name) {
            let args = tokens[1..].join(", ");
            return format!("{}({})", name, args);
        }
    }
    s.to_string()
}

fn convert_type(t: &str) -> &str {
    match t.trim() {
        "int" => "i32",
        "float" => "f64",
        "str" | "string" => "String",
        "bool" => "bool",
        "list" => "Vec<String>",
        x => x,
    }
}

fn convert_params(p: &str) -> String {
    p.replace(":int", ": i32").replace(":float", ": f64").replace(":str", ": &str").replace(":bool", ": bool")
}

fn leading_indent(s: &str) -> usize {
    s.chars().take_while(|c| *c==' ').count()
}
