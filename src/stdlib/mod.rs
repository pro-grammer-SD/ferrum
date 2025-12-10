pub mod math;
pub mod random;
pub mod jsonmod;
pub mod timemod;
pub mod sys;
pub mod osmod;
pub mod iomod;
pub mod iced_stub;
pub mod opencv;
pub mod check;
pub mod subprocess;

use crate::runtime::{Env, Function, Value};
use crate::ui;

pub fn register_stdlib(env: &mut Env) {
    // math
    env.set_func("sin".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(a.sin()))
    }));

    env.set_func("cos".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(a.cos()))
    }));

    env.set_func("tan".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(a.tan()))
    }));

    env.set_func("sqrt".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(a.sqrt()))
    }));

    env.set_func("pow".to_string(), Function::Native(|args| {
        let base = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        let exp = match args.get(1) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(base.powf(exp)))
    }));

    env.set_func("exp".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(a.exp()))
    }));

    env.set_func("ln".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(a.ln()))
    }));

    env.set_func("log".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        let base = match args.get(1) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 10.0,
        };
        Ok(Value::Float(a.log(base)))
    }));

    env.set_func("log10".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Float(f)) => *f,
            Some(Value::Int(i)) => *i as f64,
            _ => 0.0,
        };
        Ok(Value::Float(a.log10()))
    }));

    // random.randint
    env.set_func("randint".to_string(), Function::Native(|args| {
        let a = match args.get(0) {
            Some(Value::Int(i)) => *i as i64,
            _ => 0,
        };
        let b = match args.get(1) {
            Some(Value::Int(i)) => *i as i64,
            _ => 0,
        };
        let v = rand::random::<u32>() % ((b - a + 1) as u32);
        Ok(Value::Int((a + v as i64) as i64))
    }));

    // range(start, end) or range(end) -> Range(start, end)
    env.set_func("range".to_string(), Function::Native(|args| {
        if args.len() == 1 {
            match args.get(0) {
                Some(Value::Int(end)) => Ok(Value::Range(0, *end)),
                _ => Ok(Value::Range(0, 0)),
            }
        } else if args.len() >= 2 {
            match (args.get(0), args.get(1)) {
                (Some(Value::Int(s)), Some(Value::Int(e))) => Ok(Value::Range(*s, *e)),
                _ => Ok(Value::Range(0, 0)),
            }
        } else {
            Ok(Value::Range(0, 0))
        }
    }));

    // json: dumps/loads via serde_json
    env.set_func("json_dumps".to_string(), Function::Native(|args| {
        if let Some(Value::Str(s)) = args.get(0) {
            // assume s is JSON serializable string
            Ok(Value::Str(s.clone()))
        } else {
            Ok(Value::Str(String::new()))
        }
    }));

    // time.time and time.time_ms
    env.set_func("time".to_string(), Function::Native(|_args| {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        Ok(Value::Int(timestamp))
    }));

    env.set_func("time_ms".to_string(), Function::Native(|_args| {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        Ok(Value::Int(timestamp))
    }));

    // time.sleep
    env.set_func("sleep".to_string(), Function::Native(|args| {
        if let Some(Value::Float(f)) = args.get(0) {
            std::thread::sleep(std::time::Duration::from_secs_f64(*f));
        } else if let Some(Value::Int(i)) = args.get(0) {
            std::thread::sleep(std::time::Duration::from_secs(*i as u64));
        }
        Ok(Value::None)
    }));

    // sys.argv
    env.set("argv".to_string(), Value::List(vec![]));

    // os.listdir stub
    env.set_func("listdir".to_string(), Function::Native(|args| {
        if let Some(Value::Str(p)) = args.get(0) {
            let mut out = vec![];
            if let Ok(entries) = std::fs::read_dir(p) {
                for e in entries.flatten() {
                    if let Some(name) = e.file_name().to_str() {
                        out.push(Value::Str(name.to_string()));
                    }
                }
            }
            return Ok(Value::List(out));
        }
        Ok(Value::List(vec![]))
    }));

    // os.getcwd
    env.set_func("getcwd".to_string(), Function::Native(|_args| {
        match std::env::current_dir() {
            Ok(path) => {
                if let Some(path_str) = path.to_str() {
                    Ok(Value::Str(path_str.to_string()))
                } else {
                    Ok(Value::Str(".".to_string()))
                }
            }
            Err(_) => Ok(Value::Str(".".to_string())),
        }
    }));

    // os.platform
    env.set_func("platform".to_string(), Function::Native(|_args| {
        Ok(Value::Str(std::env::consts::OS.to_string()))
    }));

    // io.read / io.write
    env.set_func("read_file".to_string(), Function::Native(|args| {
        if let Some(Value::Str(p)) = args.get(0) {
            if let Ok(s) = std::fs::read_to_string(p) {
                return Ok(Value::Str(s));
            }
        }
        Ok(Value::Str(String::new()))
    }));

    env.set_func("write_file".to_string(), Function::Native(|args| {
        if let (Some(Value::Str(p)), Some(Value::Str(content))) = (args.get(0), args.get(1)) {
            let _ = std::fs::write(p, content);
        }
        Ok(Value::None)
    }));

    // Type casting and built-ins
    env.set_func("int".to_string(), Function::Native(|args| {
        if let Some(Value::Str(s)) = args.get(0) {
            if let Ok(i) = s.parse::<i64>() { return Ok(Value::Int(i)); }
        }
        if let Some(Value::Float(f)) = args.get(0) { return Ok(Value::Int(*f as i64)); }
        if let Some(Value::Int(i)) = args.get(0) { return Ok(Value::Int(*i)); }
        Ok(Value::Int(0))
    }));

    env.set_func("float".to_string(), Function::Native(|args| {
        if let Some(Value::Str(s)) = args.get(0) {
            if let Ok(f) = s.parse::<f64>() { return Ok(Value::Float(f)); }
        }
        if let Some(Value::Int(i)) = args.get(0) { return Ok(Value::Float(*i as f64)); }
        if let Some(Value::Float(f)) = args.get(0) { return Ok(Value::Float(*f)); }
        Ok(Value::Float(0.0))
    }));

    env.set_func("str".to_string(), Function::Native(|args| {
        if let Some(Value::Str(s)) = args.get(0) { return Ok(Value::Str(s.clone())); }
        if let Some(Value::Int(i)) = args.get(0) { return Ok(Value::Str(i.to_string())); }
        if let Some(Value::Float(f)) = args.get(0) { return Ok(Value::Str(f.to_string())); }
        if let Some(Value::Bool(b)) = args.get(0) { return Ok(Value::Str(b.to_string())); }
        Ok(Value::Str(String::new()))
    }));

    env.set_func("bool".to_string(), Function::Native(|args| {
        if let Some(Value::Bool(b)) = args.get(0) { return Ok(Value::Bool(*b)); }
        if let Some(Value::Int(i)) = args.get(0) { return Ok(Value::Bool(*i != 0)); }
        if let Some(Value::Str(s)) = args.get(0) { return Ok(Value::Bool(!s.is_empty())); }
        Ok(Value::Bool(false))
    }));

    env.set_func("list".to_string(), Function::Native(|args| {
        if let Some(Value::Str(s)) = args.get(0) {
            return Ok(Value::List(s.chars().map(|c| Value::Str(c.to_string())).collect()));
        }
        if let Some(Value::List(l)) = args.get(0) { return Ok(Value::List(l.clone())); }
        Ok(Value::List(vec![]))
    }));

    env.set_func("len".to_string(), Function::Native(|args| {
        if let Some(Value::Str(s)) = args.get(0) { return Ok(Value::Int(s.len() as i64)); }
        if let Some(Value::List(l)) = args.get(0) { return Ok(Value::Int(l.len() as i64)); }
        Ok(Value::Int(0))
    }));

    env.set_func("input".to_string(), Function::Native(|args| {
        use std::io::{self, Write};
        if let Some(Value::Str(prompt)) = args.get(0) { print!("{}", prompt); let _ = io::stdout().flush(); }
        let mut s = String::new();
        let _ = std::io::stdin().read_line(&mut s);
        Ok(Value::Str(s.trim_end().to_string()))
    }));

    env.set_func("abs".to_string(), Function::Native(|args| {
        if let Some(Value::Int(i)) = args.get(0) { return Ok(Value::Int(i.abs())); }
        if let Some(Value::Float(f)) = args.get(0) { return Ok(Value::Float(f.abs())); }
        Ok(Value::Int(0))
    }));

    env.set_func("zip".to_string(), Function::Native(|args| {
        // zip two lists into a list of pairs (as lists)
        if args.len() >= 2 {
            if let (Value::List(a), Value::List(b)) = (args.get(0).cloned().unwrap_or(Value::List(vec![])), args.get(1).cloned().unwrap_or(Value::List(vec![]))) {
                let n = std::cmp::min(a.len(), b.len());
                let mut out = vec![];
                for i in 0..n {
                    out.push(Value::List(vec![a[i].clone(), b[i].clone()]));
                }
                return Ok(Value::List(out));
            }
        }
        Ok(Value::List(vec![]))
    }));

    env.set_func("isdigit".to_string(), Function::Native(|args| {
        if let Some(Value::Str(s)) = args.get(0) { return Ok(Value::Bool(s.chars().all(|c| c.is_ascii_digit()))); }
        Ok(Value::Bool(false))
    }));

    // iced window/button: if compiled with `real-iced` feature, use the real backend,
    // otherwise use the stubbed implementation.
    env.set_func("iced_window".to_string(), Function::Native(|args| {
        if let Some(Value::Str(title)) = args.get(0) {
            #[cfg(feature = "real-iced")]
            {
                // Create a temporary window in the registry and launch it
                let obj = ui::UiObj::Window(ui::WindowObj { title: title.clone(), children: vec![], positions: std::collections::HashMap::new() });
                let window_id = ui::insert(obj);
                crate::stdlib::iced_stub::iced_real::launch_window(&window_id, &title);
            }
            #[cfg(not(feature = "real-iced"))]
            {
                let mut w = crate::stdlib::iced_stub::iced::Window::new();
                w.set_title(&title);
                w.run();
            }
        }
        Ok(Value::None)
    }));

    env.set_func("iced_button".to_string(), Function::Native(|args| {
        if let Some(Value::Str(label)) = args.get(0) {
            #[cfg(feature = "real-iced")]
            {
                crate::stdlib::iced_stub::iced_real::create_button(&label);
            }
            #[cfg(not(feature = "real-iced"))]
            {
                let _ = crate::stdlib::iced_stub::iced::Button::new(&label);
            }
        }
        Ok(Value::None)
    }));

    // UI system: Window, Button, Slider, RadioButton, Column, Row constructors
    fn ui_window_ctor(_args: Vec<Value>) -> anyhow::Result<Value> {
        let obj = ui::UiObj::Window(ui::WindowObj { title: String::new(), children: vec![], positions: std::collections::HashMap::new() });
        let id = ui::insert(obj);
        let mut fields = std::collections::HashMap::new();
        fields.insert("__id".to_string(), Value::Str(id.clone()));
        let mut methods = std::collections::HashMap::new();
        methods.insert("set_title".to_string(), Function::Native(ui_set_title));
        methods.insert("set_position".to_string(), Function::Native(ui_window_set_position));
        methods.insert("set_size".to_string(), Function::Native(ui_set_size));
        methods.insert("set_icon".to_string(), Function::Native(ui_set_icon));
        methods.insert("add".to_string(), Function::Native(ui_add_child));
        methods.insert("run".to_string(), Function::Native(ui_run));
        Ok(Value::Instance("Window".to_string(), fields, methods))
    }

    fn ui_button_ctor(args: Vec<Value>) -> anyhow::Result<Value> {
        let label = if let Some(Value::Str(s)) = args.get(0) { s.clone() } else { String::new() };
        let obj = ui::UiObj::Button(ui::ButtonObj { label: label.clone() });
        let id = ui::insert(obj);
        let mut fields = std::collections::HashMap::new();
        fields.insert("__id".to_string(), Value::Str(id.clone()));
        let mut methods = std::collections::HashMap::new();
        methods.insert("get_label".to_string(), Function::Native(ui_button_get_label));
        methods.insert("set_position".to_string(), Function::Native(ui_button_set_position));
        Ok(Value::Instance("Button".to_string(), fields, methods))
    }

    fn ui_slider_ctor(args: Vec<Value>) -> anyhow::Result<Value> {
        let min = if let Some(Value::Int(i)) = args.get(0) { *i } else { 0 };
        let max = if let Some(Value::Int(i)) = args.get(1) { *i } else { 100 };
        let obj = ui::UiObj::Slider(ui::SliderObj { min, max });
        let id = ui::insert(obj);
        let mut fields = std::collections::HashMap::new();
        fields.insert("__id".to_string(), Value::Str(id.clone()));
        let mut methods = std::collections::HashMap::new();
        methods.insert("set_coordinates".to_string(), Function::Native(ui_slider_set_coords));
        Ok(Value::Instance("Slider".to_string(), fields, methods))
    }

    fn ui_radio_ctor(args: Vec<Value>) -> anyhow::Result<Value> {
        let label = if let Some(Value::Str(s)) = args.get(0) { s.clone() } else { String::new() };
        let obj = ui::UiObj::Radio(ui::RadioObj { label: label.clone() });
        let id = ui::insert(obj);
        let mut fields = std::collections::HashMap::new();
        fields.insert("__id".to_string(), Value::Str(id.clone()));
        let mut methods = std::collections::HashMap::new();
        methods.insert("is_selected".to_string(), Function::Native(ui_radio_is_selected));
        methods.insert("set_position".to_string(), Function::Native(ui_radio_set_position));
        Ok(Value::Instance("RadioButton".to_string(), fields, methods))
    }

    fn ui_button_get_label(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let Some(obj) = ui::get(id) {
                        if let ui::UiObj::Button(b) = obj {
                            return Ok(Value::Str(b.label));
                        }
                    }
                }
            }
        }
        Ok(Value::Str(String::new()))
    }

    fn ui_button_set_position(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let (Some(crate::runtime::Value::Int(x)), Some(crate::runtime::Value::Int(y))) = (args.get(1), args.get(2)) {
                        ui::set_position(id, &format!("pos"), *x, *y);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_slider_set_coords(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let (Some(crate::runtime::Value::Int(x)), Some(crate::runtime::Value::Int(y))) = (args.get(1), args.get(2)) {
                        ui::set_position(id, &format!("coords"), *x, *y);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_radio_is_selected(_args: Vec<Value>) -> anyhow::Result<Value> {
        // For now, always return false - real implementation would check state
        Ok(Value::Bool(false))
    }

    fn ui_radio_set_position(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let (Some(crate::runtime::Value::Int(x)), Some(crate::runtime::Value::Int(y))) = (args.get(1), args.get(2)) {
                        ui::set_position(id, &format!("pos"), *x, *y);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_layout_set_position(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let (Some(crate::runtime::Value::Int(x)), Some(crate::runtime::Value::Int(y))) = (args.get(1), args.get(2)) {
                        ui::set_position(id, &format!("layout"), *x, *y);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_layout_set_spacing(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(_selfv) = args.get(0) {
            // For now, just accept the spacing value - it's not stored yet
            if args.get(1).is_some() {
                // spacing is accepted but not used in stub mode
            }
        }
        Ok(Value::None)
    }

    fn ui_layout_add_child(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let Some(crate::runtime::Value::Str(child_id)) = args.get(1) {
                        ui::set_position(id, child_id, 0, 0);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_column_ctor(_args: Vec<Value>) -> anyhow::Result<Value> {
        let obj = ui::UiObj::Column(ui::ColumnObj { children: vec![] });
        let id = ui::insert(obj);
        let mut fields = std::collections::HashMap::new();
        fields.insert("__id".to_string(), Value::Str(id.clone()));
        let mut methods = std::collections::HashMap::new();
        methods.insert("set_position".to_string(), Function::Native(ui_layout_set_position));
        methods.insert("set_spacing".to_string(), Function::Native(ui_layout_set_spacing));
        methods.insert("add".to_string(), Function::Native(ui_layout_add_child));
        Ok(Value::Instance("Column".to_string(), fields, methods))
    }

    fn ui_row_ctor(_args: Vec<Value>) -> anyhow::Result<Value> {
        let obj = ui::UiObj::Row(ui::RowObj { children: vec![] });
        let id = ui::insert(obj);
        let mut fields = std::collections::HashMap::new();
        fields.insert("__id".to_string(), Value::Str(id.clone()));
        let mut methods = std::collections::HashMap::new();
        methods.insert("set_position".to_string(), Function::Native(ui_layout_set_position));
        methods.insert("set_spacing".to_string(), Function::Native(ui_layout_set_spacing));
        methods.insert("add".to_string(), Function::Native(ui_layout_add_child));
        Ok(Value::Instance("Row".to_string(), fields, methods))
    }

    // native methods operate on 'self' as first arg
    fn ui_set_title(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let Some(crate::runtime::Value::Str(title)) = args.get(1) {
                        ui::set_title(id, title);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_window_set_position(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let (Some(crate::runtime::Value::Int(x)), Some(crate::runtime::Value::Int(y))) = (args.get(1), args.get(2)) {
                        ui::set_position(id, &format!("window"), *x, *y);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_set_size(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let (Some(crate::runtime::Value::Int(w)), Some(crate::runtime::Value::Int(h))) = (args.get(1), args.get(2)) {
                        // Update the window size in the registry
                        ui::set_size(id, *w, *h);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_set_icon(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let Some(crate::runtime::Value::Str(icon_path)) = args.get(1) {
                        ui::set_icon(id, icon_path);
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_add_child(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let Some(childv) = args.get(1) {
                        if let crate::runtime::Value::Instance(_, cfields, _) = childv {
                            if let Some(crate::runtime::Value::Str(child_id)) = cfields.get("__id") {
                                ui::add_child(id, child_id);
                            }
                        }
                    }
                }
            }
        }
        Ok(Value::None)
    }

    fn ui_run(args: Vec<Value>) -> anyhow::Result<Value> {
        if let Some(selfv) = args.get(0) {
            if let crate::runtime::Value::Instance(_, fields, _) = selfv {
                if let Some(crate::runtime::Value::Str(id)) = fields.get("__id") {
                    if let Some(obj) = ui::get(id) {
                        match obj {
                            ui::UiObj::Window(_w) => {
                                #[cfg(feature = "real-iced")]
                                {
                                    // Pass both the window ID and its title to iced_real
                                    crate::stdlib::iced_stub::iced_real::launch_window(id, &_w.title);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(Value::None)
    }

    // register constructors as functions
    env.set_func("Window".to_string(), Function::Native(ui_window_ctor));
    env.set_func("Button".to_string(), Function::Native(ui_button_ctor));
    env.set_func("Slider".to_string(), Function::Native(ui_slider_ctor));
    env.set_func("RadioButton".to_string(), Function::Native(ui_radio_ctor));
    env.set_func("Column".to_string(), Function::Native(ui_column_ctor));
    env.set_func("Row".to_string(), Function::Native(ui_row_ctor));

    // OpenCV image processing functions
    env.set_func("cv_load_image".to_string(), Function::Native(|args| {
        if let Some(Value::Str(path)) = args.get(0) {
            Ok(Value::Str(opencv::opencv::load_image(path)))
        } else {
            Ok(Value::Str("[OpenCV] Error: path required".to_string()))
        }
    }));

    env.set_func("cv_display".to_string(), Function::Native(|args| {
        if let (Some(Value::Str(window)), Some(Value::Str(img_id))) = (args.get(0), args.get(1)) {
            Ok(Value::Str(opencv::opencv::display_image(window, img_id)))
        } else {
            Ok(Value::Str("[OpenCV] Error: window and image_id required".to_string()))
        }
    }));

    env.set_func("cv_gaussian_blur".to_string(), Function::Native(|args| {
        if let (Some(Value::Str(img_id)), Some(Value::Int(kernel))) = (args.get(0), args.get(1)) {
            Ok(Value::Str(opencv::opencv::gaussian_blur(img_id, *kernel as i32)))
        } else {
            Ok(Value::Str("[OpenCV] Error: image_id and kernel_size required".to_string()))
        }
    }));

    env.set_func("cv_draw_rect".to_string(), Function::Native(|args| {
        if args.len() >= 9 {
            if let (Some(Value::Str(img)), Some(Value::Int(x1)), Some(Value::Int(y1)), 
                    Some(Value::Int(x2)), Some(Value::Int(y2)), Some(Value::Int(r)), 
                    Some(Value::Int(g)), Some(Value::Int(b)), Some(Value::Int(t))) = 
                   (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4),
                    args.get(5), args.get(6), args.get(7), args.get(8)) {
                Ok(Value::Str(opencv::opencv::draw_rectangle(img, *x1 as i32, *y1 as i32, 
                    *x2 as i32, *y2 as i32, *r as i32, *g as i32, *b as i32, *t as i32)))
            } else {
                Ok(Value::Str("[OpenCV] Error: invalid arguments".to_string()))
            }
        } else {
            Ok(Value::Str("[OpenCV] Error: 9 arguments required".to_string()))
        }
    }));

    env.set_func("cv_draw_circle".to_string(), Function::Native(|args| {
        if args.len() >= 9 {
            if let (Some(Value::Str(img)), Some(Value::Int(cx)), Some(Value::Int(cy)), 
                    Some(Value::Int(rad)), Some(Value::Int(r)), Some(Value::Int(g)), 
                    Some(Value::Int(b)), Some(Value::Int(t)), _) = 
                   (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4),
                    args.get(5), args.get(6), args.get(7), args.get(8)) {
                Ok(Value::Str(opencv::opencv::draw_circle(img, *cx as i32, *cy as i32, 
                    *rad as i32, *r as i32, *g as i32, *b as i32, *t as i32)))
            } else {
                Ok(Value::Str("[OpenCV] Error: invalid arguments".to_string()))
            }
        } else {
            Ok(Value::Str("[OpenCV] Error: 8 arguments required".to_string()))
        }
    }));

    env.set_func("cv_draw_line".to_string(), Function::Native(|args| {
        if args.len() >= 9 {
            if let (Some(Value::Str(img)), Some(Value::Int(x1)), Some(Value::Int(y1)), 
                    Some(Value::Int(x2)), Some(Value::Int(y2)), Some(Value::Int(r)), 
                    Some(Value::Int(g)), Some(Value::Int(b)), Some(Value::Int(t))) = 
                   (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4),
                    args.get(5), args.get(6), args.get(7), args.get(8)) {
                Ok(Value::Str(opencv::opencv::draw_line(img, *x1 as i32, *y1 as i32, 
                    *x2 as i32, *y2 as i32, *r as i32, *g as i32, *b as i32, *t as i32)))
            } else {
                Ok(Value::Str("[OpenCV] Error: invalid arguments".to_string()))
            }
        } else {
            Ok(Value::Str("[OpenCV] Error: 9 arguments required".to_string()))
        }
    }));

    env.set_func("cv_draw_text".to_string(), Function::Native(|args| {
        if args.len() >= 8 {
            if let (Some(Value::Str(img)), Some(Value::Str(text)), Some(Value::Int(x)), 
                    Some(Value::Int(y)), Some(Value::Int(r)), Some(Value::Int(g)), 
                    Some(Value::Int(b)), Some(Value::Float(scale))) = 
                   (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4),
                    args.get(5), args.get(6), args.get(7)) {
                Ok(Value::Str(opencv::opencv::draw_text(img, text, *x as i32, *y as i32, 
                    *r as i32, *g as i32, *b as i32, *scale as f32)))
            } else {
                Ok(Value::Str("[OpenCV] Error: invalid arguments".to_string()))
            }
        } else {
            Ok(Value::Str("[OpenCV] Error: 8 arguments required".to_string()))
        }
    }));

    env.set_func("cv_start_camera".to_string(), Function::Native(|_args| {
        Ok(Value::Str(opencv::opencv::start_camera()))
    }));

    env.set_func("cv_detect_faces".to_string(), Function::Native(|args| {
        if let Some(Value::Str(img_id)) = args.get(0) {
            let cascade_path = match args.get(1) {
                Some(Value::Str(p)) => p.clone(),
                _ => "haarcascade_frontalface_default.xml".to_string(),
            };
            let faces = opencv::opencv::detect_faces(img_id, &cascade_path);
            let face_list: Vec<Value> = faces.into_iter()
                .map(|(x, y, w, h)| Value::List(vec![
                    Value::Int(x as i64),
                    Value::Int(y as i64),
                    Value::Int(w as i64),
                    Value::Int(h as i64),
                ]))
                .collect();
            Ok(Value::List(face_list))
        } else {
            Ok(Value::List(vec![]))
        }
    }));

    env.set_func("cv_detect_body_landmarks".to_string(), Function::Native(|args| {
        if let Some(Value::Str(img_id)) = args.get(0) {
            let landmarks = opencv::opencv::detect_body_landmarks(img_id);
            let landmark_list: Vec<Value> = landmarks.into_iter()
                .map(|(x, y, name)| Value::List(vec![
                    Value::Int(x as i64),
                    Value::Int(y as i64),
                    Value::Str(name),
                ]))
                .collect();
            Ok(Value::List(landmark_list))
        } else {
            Ok(Value::List(vec![]))
        }
    }));

    env.set_func("cv_detect_hand_keypoints".to_string(), Function::Native(|args| {
        if let Some(Value::Str(img_id)) = args.get(0) {
            let keypoints = opencv::opencv::detect_hand_keypoints(img_id);
            let kp_list: Vec<Value> = keypoints.into_iter()
                .map(|(x, y, name)| Value::List(vec![
                    Value::Int(x as i64),
                    Value::Int(y as i64),
                    Value::Str(name),
                ]))
                .collect();
            Ok(Value::List(kp_list))
        } else {
            Ok(Value::List(vec![]))
        }
    }));

    env.set_func("cv_save_image".to_string(), Function::Native(|args| {
        if let (Some(Value::Str(img_id)), Some(Value::Str(path))) = (args.get(0), args.get(1)) {
            Ok(Value::Str(opencv::opencv::save_image(img_id, path)))
        } else {
            Ok(Value::Str("[OpenCV] Error: image_id and path required".to_string()))
        }
    }));

    // Code analysis and checking function
    env.set_func("check".to_string(), Function::Native(|args| {
        if let Some(Value::Str(source)) = args.get(0) {
            match check::check_script(source) {
                Ok(result) => {
                    let mut output = vec![result.summary];
                    for error in &result.errors {
                        output.push(format!("  ERROR [{}] Line {}: {}", error.code, error.line, error.message));
                    }
                    for warning in &result.warnings {
                        output.push(format!("  WARNING [{}] Line {}: {}", warning.code, warning.line, warning.message));
                    }
                    Ok(Value::Str(output.join("\n")))
                }
                Err(e) => Ok(Value::Str(format!("Check failed: {}", e)))
            }
        } else {
            Ok(Value::Str("[Check] Error: source code string required".to_string()))
        }
    }));

    // Subprocess module functions
    env.set_func("subprocess_run".to_string(), Function::Native(crate::stdlib::subprocess::subprocess_run));
    env.set_func("subprocess_popen".to_string(), Function::Native(crate::stdlib::subprocess::subprocess_popen));

    // Help system function
    env.set_func("help".to_string(), Function::Native(|args| {
        if let Some(Value::Str(topic)) = args.get(0) {
            match topic.as_str() {
                "print" => Ok(Value::Str("print(value) - Prints a value to standard output\n  Example: print(\"Hello, World!\")".to_string())),
                "len" => Ok(Value::Str("len(obj) - Returns the length of a string or list\n  Example: len(\"hello\") returns 5".to_string())),
                "range" => Ok(Value::Str("range(end) or range(start, end) - Creates a range object\n  Example: for i in range(1, 5): print(i)".to_string())),
                "str" | "int" | "float" | "bool" => Ok(Value::Str(format!("{}(obj) - Converts object to {} type", topic, topic))),
                "sin" | "cos" | "sqrt" => Ok(Value::Str(format!("{}(x) - Math function\n  Example: {} = {}(1.57)", topic, topic, topic))),
                "subprocess_run" => Ok(Value::Str("subprocess_run(cmd) - Execute command and wait for completion\n  Returns dict with stdout, stderr, exit_code\n  Example: result = subprocess_run(\"echo hello\")".to_string())),
                "subprocess_popen" => Ok(Value::Str("subprocess_popen(cmd) - Spawn process for live interaction\n  Returns dict with pid\n  Example: proc = subprocess_popen(\"ping localhost\")".to_string())),
                _ => {
                    let help_text = format!(
                        "No help available for '{}'\n\nAvailable topics:\n  print, len, range, str, int, float, bool\n  sin, cos, sqrt, pow, abs, exp\n  read_file, write_file, input\n  subprocess_run, subprocess_popen\n  Window, Button, Slider, RadioButton\n\nFor OpenCV: cv_load_image, cv_gaussian_blur, cv_draw_rect, cv_draw_circle, cv_detect_faces",
                        topic
                    );
                    Ok(Value::Str(help_text))
                }
            }
        } else {
            Ok(Value::Str("help(topic) - Get help on a function or feature\n  Example: help(\"print\")\n\nUse help() with no args to see common topics".to_string()))
        }
    }));
}
