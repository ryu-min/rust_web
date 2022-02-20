use rand::prelude::*;
use std::env;

mod to_do;

use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

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

    let done: Done = Done::new(String::from("shopping"));
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending: Pending = Pending::new(String::from("laundry"));
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);

    // let args: Vec<String> = env::args().collect();
    // let path: &str = &args[0];
    // if path .contains("/debug/") {
    //     println!("The development app is running");
    // }
    // else if path.contains("/release/"){
    //     println!("The production server is running");
    // }
    // else {
    //     panic!("The settings is neither debug or release");
    // }
    //
    // let mut rng: ThreadRng = rand::thread_rng();
    // let random_number = generate_float(&mut rng);
    // println!("{}", random_number);
}
