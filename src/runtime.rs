use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;

/// Runtime value types in Ferrum
/// 
/// Ferrum is dynamically typed and supports the following value types:
/// - Integers (i64)
/// - Floats (f64)
/// - Booleans (bool)
/// - Strings (String)
/// - Lists (Vec<Value>)
/// - Dictionaries (HashMap<String, Value>)
/// - Classes and Instances
/// - Ranges for iteration
/// - None (nil/null)
#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
        Class(String, HashMap<String, Function>),
        Instance(String, HashMap<String, Value>, HashMap<String, Function>),
    None,
    Range(i64, i64),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::List(vs) => !vs.is_empty(),
            Value::Dict(m) => !m.is_empty(),
            Value::None => false,
            Value::Range(_, _) => true,
            Value::Class(_, _) => true,
            Value::Instance(_, _, _) => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::List(vs) => format!("[{}]", vs.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")),
            Value::Dict(_) => "{...}".to_string(),
            Value::Class(name, _) => format!("<class {}>", name),
            Value::Instance(name, _, _) => format!("<instance {}>", name),
            Value::None => "None".to_string(),
            Value::Range(s,e) => format!("range({}, {})", s, e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Native(fn(Vec<Value>) -> Result<Value>),
    User { params: Vec<String>, body: Vec<crate::eval::Stmt> },
}

/// Runtime environment containing variables, functions, and modules
/// 
/// The environment maintains state during script execution including:
/// - Variable bindings (vars)
/// - Function definitions (funcs)
/// - Loaded modules with their exports (modules)
/// 
/// Each function call creates a new child environment to handle local scope.
pub struct Env {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, Function>,
    pub modules: HashMap<String, HashMap<String, Value>>,
}

impl Env {
    pub fn new() -> Self {
        Env { vars: HashMap::new(), funcs: HashMap::new(), modules: HashMap::new() }
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn set_func(&mut self, name: String, f: Function) {
        self.funcs.insert(name, f);
    }

    pub fn call(&mut self, name: &str, args: Vec<Value>) -> Result<Value> {
        // direct function call
        if let Some(f) = self.funcs.get(name) {
            match f {
                Function::Native(fun) => fun(args),
                Function::User { params, body } => {
                    // create new local env
                    let mut child = Env::new();
                    // pass args positionally
                    for (i, p) in params.iter().enumerate() {
                        if let Some(v) = args.get(i) {
                            child.set(p.clone(), v.clone());
                        } else {
                            child.set(p.clone(), Value::None);
                        }
                    }
                    // inherit std modules/functions
                    // run body
                    crate::eval::run_stmts(body, &mut child)?;
                    Ok(Value::None)
                }
            }
        } else if let Some(v) = self.vars.get(name) {
            // If it's a class, calling it constructs an instance
            match v {
                Value::Class(class_name, methods) => {
                    let fields = HashMap::new();
                    let inst_methods = methods.clone();

                    // create instance and child env to run __init__ if present
                    let mut child = Env::new();
                    let instance = Value::Instance(class_name.clone(), fields, inst_methods.clone());
                    child.set("self".to_string(), instance.clone());

                    if let Some(init_fn) = inst_methods.get("__init__") {
                        match init_fn {
                            Function::User { params, body } => {
                                // bind positional args to params (skipping self)
                                let mut ai = 0usize;
                                for p in params.iter() {
                                    if p == "self" {
                                        // self already set
                                        continue;
                                    }
                                    if let Some(a) = args.get(ai) {
                                        child.set(p.clone(), a.clone());
                                    } else {
                                        child.set(p.clone(), Value::None);
                                    }
                                    ai += 1;
                                }
                                crate::eval::run_stmts(body, &mut child)?;
                            }
                            _ => {}
                        }
                    }

                    // retrieve possibly-updated instance from child
                    if let Some(v) = child.get("self") {
                        return Ok(v.clone());
                    }

                    Ok(Value::Instance(class_name.clone(), HashMap::new(), inst_methods))
                }
                _ => Err(anyhow!("Undefined function: {}", name)),
            }
        } else {
            Err(anyhow!("Undefined function: {}", name))
        }
    }

    pub fn import_module(&mut self, name: &str) -> Result<()> {
        let path = if name.ends_with(".fm") { name.to_string() } else { format!("{}.fm", name) };
        let p = Path::new(&path);
        if !p.exists() {
            // If a module file isn't present, try to expose a builtin stdlib module
            // by creating a Dict mapping attribute names to function names so scripts
            // can do `import iced` and call `iced.Window()` etc.
            let mut module_map = HashMap::new();
            // Export all currently-registered functions into the module as string refs
            for k in self.funcs.keys() {
                module_map.insert(k.clone(), Value::Str(k.clone()));
            }
            // Also export any top-level vars
            for (k, v) in self.vars.iter() {
                module_map.insert(k.clone(), v.clone());
            }
            // Insert into modules and also create a module variable so `env.get(name)` works
            self.modules.insert(name.to_string(), module_map.clone());
            self.set(name.to_string(), Value::Dict(module_map));
            return Ok(());
        }
        let src = std::fs::read_to_string(p)?;
        // create module env and run
        let mut modenv = Env::new();
        super::stdlib::register_stdlib(&mut modenv);
        super::parser::parse_and_run(&src, &mut modenv)?;
        // collect exported vars
        let mut module_map = HashMap::new();
        for (k, v) in modenv.vars.into_iter() {
            module_map.insert(k, v);
        }
        self.modules.insert(name.to_string(), module_map);
        Ok(())
    }

    pub fn set_attr(&mut self, obj: &str, attr: &str, val: Value) -> Result<()> {
        if let Some(existing) = self.vars.get_mut(obj) {
            match existing {
                Value::Instance(_cls, fields, _methods) => {
                    fields.insert(attr.to_string(), val);
                    return Ok(());
                }
                Value::Dict(map) => {
                    map.insert(attr.to_string(), val);
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(anyhow!("Cannot set attribute {} on {}", attr, obj))
    }
}
