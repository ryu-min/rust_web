use serde_json::{json, value::Value, Map};

use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title : &str, state : &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file("/home/z1p1t/workspace/rust_web/state.json", state);
        println!("\n\n{} is being set to done\n\n", title);
    }

    fn set_to_pending(&self, title : &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file("/home/z1p1t/workspace/rust_web/state.json", state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
}