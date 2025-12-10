/// Code Analysis and Checking Module for Ferrum
/// 
/// Provides static analysis capabilities for Ferrum scripts:
/// - Syntax validation
/// - Type checking (basic)
/// - Unused variable warnings
/// - Deprecated function detection
/// - Function signature validation

use std::collections::HashSet;
use anyhow::Result;

/// Result of analyzing a Ferrum script
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub errors: Vec<AnalysisError>,
    pub warnings: Vec<AnalysisWarning>,
    pub summary: String,
}

/// An error found during analysis
#[derive(Debug, Clone)]
pub struct AnalysisError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub code: String, // Error code like "E001"
}

/// A warning found during analysis
#[derive(Debug, Clone)]
pub struct AnalysisWarning {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub code: String, // Warning code like "W001"
}

/// Analyze a Ferrum script for errors and issues
/// 
/// # Arguments
/// * `source` - The Ferrum script source code
/// 
/// # Returns
/// CheckResult with all found errors and warnings
/// 
/// # Example
/// ```ignore
/// let result = check_script("x = 1\nprint(x)")?;
/// println!("Errors: {}", result.errors.len());
/// println!("Warnings: {}", result.warnings.len());
/// ```
pub fn check_script(source: &str) -> Result<CheckResult> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    
    // Try to parse - syntax errors
    if let Err(parse_err) = crate::parser::parse_to_ast(source) {
        errors.push(AnalysisError {
            line: 1,
            column: 0,
            message: format!("Syntax Error: {}", parse_err),
            code: "E001".to_string(),
        });
        let summary = format!("Script analysis found {} error(s)", errors.len());
        return Ok(CheckResult { errors, warnings, summary });
    }
    
    // Check for common issues
    check_undefined_variables(source, &mut warnings);
    check_deprecated_functions(source, &mut warnings);
    check_unused_variables(source, &mut warnings);
    check_type_mismatches(source, &mut warnings);
    
    let error_count = errors.len();
    let warning_count = warnings.len();
    let summary = if error_count == 0 && warning_count == 0 {
        "No issues found - script looks good!".to_string()
    } else {
        format!("Script analysis found {} error(s) and {} warning(s)", error_count, warning_count)
    };
    
    Ok(CheckResult { errors, warnings, summary })
}

/// Check for potentially undefined variables
fn check_undefined_variables(source: &str, warnings: &mut Vec<AnalysisWarning>) {
    let mut defined = HashSet::new();
    defined.insert("self".to_string());
    
    // Add built-in functions
    let builtins = vec![
        "print", "len", "str", "int", "float", "bool", "list", "range",
        "sin", "cos", "tan", "sqrt", "pow", "exp", "abs",
        "read_file", "write_file", "input", "randint",
        "time", "sleep", "getcwd", "platform", "listdir",
        "zip", "isdigit"
    ];
    
    for builtin in builtins {
        defined.insert(builtin.to_string());
    }
    
    // Simple check: look for assignments
    for line_num in source.lines().enumerate() {
        let (idx, line) = line_num;
        let trimmed = line.trim();
        
        if trimmed.contains('=') && !trimmed.starts_with("#") {
            if let Some((lhs, _)) = trimmed.split_once('=') {
                let var_name = lhs.trim().split_whitespace().next().unwrap_or("");
                if !var_name.is_empty() && !var_name.contains('(') {
                    defined.insert(var_name.to_string());
                }
            }
        }
        
        // Look for undefined variables
        if trimmed.starts_with("print(") {
            if let Some(content) = trimmed.strip_prefix("print(").and_then(|s| s.strip_suffix(")")) {
                if content.starts_with('"') == false && content.starts_with('\'') == false {
                    // Potential variable reference
                    let var_name = content.split_whitespace().next().unwrap_or("");
                    if !var_name.is_empty() && !var_name.contains('"') && !defined.contains(var_name) {
                        if !var_name.contains('+') && !var_name.contains('(') {
                            warnings.push(AnalysisWarning {
                                line: idx + 1,
                                column: 0,
                                message: format!("Variable '{}' may not be defined", var_name),
                                code: "W002".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Check for usage of deprecated functions
fn check_deprecated_functions(source: &str, warnings: &mut Vec<AnalysisWarning>) {
    let deprecated = vec![
        ("print_debug", "Use print() instead"),
        ("dbg", "Use print() instead"),
        ("typeof", "Type information is implicit in Ferrum"),
    ];
    
    for (idx, line) in source.lines().enumerate() {
        for (func, suggestion) in &deprecated {
            if line.contains(func) && !line.trim().starts_with("#") {
                warnings.push(AnalysisWarning {
                    line: idx + 1,
                    column: line.find(func).unwrap_or(0),
                    message: format!("Function '{}' is deprecated: {}", func, suggestion),
                    code: "W003".to_string(),
                });
            }
        }
    }
}

/// Check for unused variables
fn check_unused_variables(source: &str, warnings: &mut Vec<AnalysisWarning>) {
    let mut assigned = std::collections::HashMap::new();
    let mut used = HashSet::new();
    
    for (idx, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("#") { continue; }
        
        // Track assignments
        if trimmed.contains('=') && !trimmed.contains("==") {
            if let Some((lhs, _)) = trimmed.split_once('=') {
                let var_name = lhs.trim().split_whitespace().next().unwrap_or("");
                if !var_name.is_empty() && !var_name.contains('(') {
                    assigned.insert(var_name.to_string(), idx + 1);
                }
            }
        }
        
        // Track usage (simple heuristic)
        for var_name in assigned.keys() {
            if line.contains(var_name) && line.find('=').map_or(true, |pos| pos > line.find(var_name).unwrap_or(0)) {
                used.insert(var_name.clone());
            }
        }
    }
    
    // Report unused
    for (var_name, line_num) in assigned {
        if !used.contains(&var_name) && !var_name.starts_with('_') {
            warnings.push(AnalysisWarning {
                line: line_num,
                column: 0,
                message: format!("Variable '{}' is assigned but never used", var_name),
                code: "W001".to_string(),
            });
        }
    }
}

/// Check for potential type mismatches
fn check_type_mismatches(source: &str, warnings: &mut Vec<AnalysisWarning>) {
    for (idx, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("#") { continue; }
        
        // Check for string + number without conversion
        if trimmed.contains("+") && !trimmed.starts_with("print") {
            let parts: Vec<&str> = trimmed.split('+').collect();
            if parts.len() >= 2 {
                for i in 0..parts.len() - 1 {
                    let left = parts[i].trim();
                    let right = parts[i + 1].trim();
                    
                    // Heuristic: if one looks like a number and the other looks like a string
                    let left_is_num = left.parse::<i32>().is_ok() || left.parse::<f64>().is_ok();
                    let right_is_num = right.parse::<i32>().is_ok() || right.parse::<f64>().is_ok();
                    let left_is_str = left.starts_with('"') || left.starts_with('\'');
                    let right_is_str = right.starts_with('"') || right.starts_with('\'');
                    
                    if (left_is_num && right_is_str) || (left_is_str && right_is_num) {
                        warnings.push(AnalysisWarning {
                            line: idx + 1,
                            column: 0,
                            message: "Possible type mismatch: mixing string and number without conversion".to_string(),
                            code: "W004".to_string(),
                        });
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid_script() {
        let result = check_script("x = 5\nprint(x)").unwrap();
        assert_eq!(result.errors.len(), 0);
    }

    #[test]
    fn test_check_syntax_error() {
        let result = check_script("x = ").unwrap();
        assert!(result.errors.len() > 0);
    }
}
