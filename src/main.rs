use rand::prelude::*;
use std::env;

mod to_do;

use to_do::ItemType;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;

fn generate_float(generator : &mut ThreadRng) -> f64  {
    let placeholder : f64 = generator.gen();
    placeholder * 10.
}

trait IsUser {
    fn is_user() -> bool {
        true
    }
}

struct User {
    name : String,
    age : i8
}

fn main() {

    let to_do_item =
        to_do_factory(String::from("pending"),
                      String::from("washing"));

    match to_do_item.unwrap() {
        ItemType::Pending(item) => item.create(&item.super_struct.title),
        ItemType::Done(item) =>
            println!("It's a done item with the title {}", item.super_struct.title)
    }
}
