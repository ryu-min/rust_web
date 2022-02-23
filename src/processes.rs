use serde_json::{Map, value::Value};

use super::to_do::ItemType;
use super::to_do::structs::{done::Done, pending::Pending};
use super::to_do::structs::traits::{get::Get, create::Create, delete::Delete, edit::Edit};

 pub fn process_pending(item : Pending, command : &str, state : &Map<String, Value>) {
     let mut state = state.clone();
     match command {
         "get" => item.get(&item.super_struct.title, &state),
         "create" => item.create(&item.super_struct.title, &item.super_struct.status, &mut state),
         "delete" => item.delete(&item.super_struct.title, &mut state),
         "edit" => item.set_to_done(&item.super_struct.title, &mut state),
         _ => println!("command '{}' is not supported", command),
     }
 }

pub fn process_done(item: Done, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command '{}' is not supported", command),
    }
}

pub fn process_input(item: ItemType, command : &str, state : &Map<String, Value>) {
    match item {
        ItemType::Pending(item) => process_pending(item, command, state),
        ItemType::Done(item) => process_done(item, command, state),
    }
}