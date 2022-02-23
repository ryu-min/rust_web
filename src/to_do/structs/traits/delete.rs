use serde_json::{Map, value::Value};

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("/home/z1p1t/workspace/rust_web/state.json", state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}