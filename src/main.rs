mod to_do;
mod state;
mod processes;

use std::env;
use serde_json::{json, Map, value::Value};

use state::{write_to_file, read_file};

use to_do::ItemType;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;

use processes::process_input;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("/home/z1p1t/workspace/rust_web/state.json");
    let status : String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        },
        None => {
            status = String::from("pending");
        }
    }

    let item = to_do_factory(&status, title).expect(&status);
    process_input(item, command, &state);
}
