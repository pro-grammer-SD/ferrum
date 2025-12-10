/// UI Registry and Object Management
/// 
/// This module maintains a global registry of UI objects created in Ferrum scripts.
/// The registry allows UI elements to be created in Ferrum code and referenced
/// by their unique IDs for later manipulation.
/// 
/// UI Objects can be:
/// - Windows (top-level containers)
/// - Buttons (interactive elements)
/// - Sliders (value input widgets)
/// - RadioButtons (selection widgets)
/// - Columns (vertical layout)
/// - Rows (horizontal layout)

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

/// UI object type enum supporting all Ferrum GUI elements
#[derive(Debug, Clone)]
pub enum UiObj {
    Window(WindowObj),
    Button(ButtonObj),
    Slider(SliderObj),
    Radio(RadioObj),
    Column(ColumnObj),
    Row(RowObj),
}

#[derive(Debug, Clone)]
pub struct WindowObj {
    pub title: String,
    pub children: Vec<String>,
    pub positions: HashMap<String, (i64, i64)>,
}

#[derive(Debug, Clone)]
pub struct ButtonObj { pub label: String }

#[derive(Debug, Clone)]
pub struct SliderObj { pub min: i64, pub max: i64 }

#[derive(Debug, Clone)]
pub struct RadioObj { pub label: String }

#[derive(Debug, Clone)]
pub struct ColumnObj { pub children: Vec<String> }

#[derive(Debug, Clone)]
pub struct RowObj { pub children: Vec<String> }

pub static REGISTRY: Lazy<Mutex<HashMap<String, UiObj>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn create_id(kind: &str) -> String {
    let n = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    format!("{}-{}", kind, n)
}

pub fn insert(obj: UiObj) -> String {
    let kind = match &obj {
        UiObj::Window(_) => "window",
        UiObj::Button(_) => "button",
        UiObj::Slider(_) => "slider",
        UiObj::Radio(_) => "radio",
        UiObj::Column(_) => "column",
        UiObj::Row(_) => "row",
    };
    let id = create_id(kind);
    let mut reg = REGISTRY.lock().unwrap();
    reg.insert(id.clone(), obj);
    id
}

pub fn get(id: &str) -> Option<UiObj> {
    let reg = REGISTRY.lock().unwrap();
    reg.get(id).cloned()
}

pub fn set_title(id: &str, title: &str) {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(obj) = reg.get_mut(id) {
        if let UiObj::Window(w) = obj {
            w.title = title.to_string();
        }
    }
}

pub fn set_position(id: &str, child_id: &str, x: i64, y: i64) {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(obj) = reg.get_mut(id) {
        match obj {
            UiObj::Window(w) => { w.positions.insert(child_id.to_string(), (x,y)); }
            _ => {}
        }
    }
}

pub fn set_size(id: &str, width: i64, height: i64) {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(obj) = reg.get_mut(id) {
        // Store width/height in positions map temporarily as a hack (0,-1) for dimensions
        match obj {
            UiObj::Window(w) => { w.positions.insert("__size".to_string(), (width, height)); }
            _ => {}
        }
    }
}

pub fn set_icon(id: &str, icon_path: &str) {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(obj) = reg.get_mut(id) {
        match obj {
            UiObj::Window(w) => { w.positions.insert("__icon".to_string(), (icon_path.len() as i64, 0)); }
            _ => {}
        }
    }
}
pub fn add_child(id: &str, child_id: &str) {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(obj) = reg.get_mut(id) {
        match obj {
            UiObj::Window(w) => { w.children.push(child_id.to_string()); }
            UiObj::Column(c) => { c.children.push(child_id.to_string()); }
            UiObj::Row(r) => { r.children.push(child_id.to_string()); }
            _ => {}
        }
    }
}
