//! JSON module - JSON serialization and deserialization
//!
//! This module provides functions to convert between Ferrum values and JSON strings.

use serde_json::Value as JsonValue;

/// Serialize a Ferrum value to JSON string
pub fn dumps(value: &crate::runtime::Value) -> String {
    match value {
        crate::runtime::Value::None => "null".to_string(),
        crate::runtime::Value::Bool(b) => b.to_string(),
        crate::runtime::Value::Int(i) => i.to_string(),
        crate::runtime::Value::Float(f) => f.to_string(),
        crate::runtime::Value::Str(s) => format!(r#""{}""#, s.replace('\\', "\\\\").replace('"', "\\\"")),
        crate::runtime::Value::List(items) => {
            let json_items: Vec<String> = items.iter().map(|v| dumps(v)).collect();
            format!("[{}]", json_items.join(","))
        }
        crate::runtime::Value::Dict(map) => {
            let mut json_items: Vec<String> = map
                .iter()
                .map(|(k, v)| format!(r#""{}":{}  "#, k, dumps(v)))
                .collect();
            json_items.sort();
            format!("{{{}}}", json_items.join(","))
        }
        _ => "null".to_string(),
    }
}

/// Deserialize JSON string to Ferrum value
pub fn loads(json_str: &str) -> Result<crate::runtime::Value, String> {
    match serde_json::from_str::<JsonValue>(json_str) {
        Ok(json_val) => Ok(json_to_ferrum_value(&json_val)),
        Err(e) => Err(format!("JSON parse error: {}", e)),
    }
}

/// Convert a serde_json Value to a Ferrum Value
fn json_to_ferrum_value(json_val: &JsonValue) -> crate::runtime::Value {
    match json_val {
        JsonValue::Null => crate::runtime::Value::None,
        JsonValue::Bool(b) => crate::runtime::Value::Bool(*b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                crate::runtime::Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                crate::runtime::Value::Float(f)
            } else {
                crate::runtime::Value::Int(0)
            }
        }
        JsonValue::String(s) => crate::runtime::Value::Str(s.clone()),
        JsonValue::Array(arr) => {
            let items: Vec<crate::runtime::Value> = arr.iter().map(json_to_ferrum_value).collect();
            crate::runtime::Value::List(items)
        }
        JsonValue::Object(obj) => {
            let mut map = std::collections::HashMap::new();
            for (k, v) in obj.iter() {
                map.insert(k.clone(), json_to_ferrum_value(v));
            }
            crate::runtime::Value::Dict(map)
        }
    }
}
