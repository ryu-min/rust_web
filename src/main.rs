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

use actix_web::{web, App, Responder, HttpServer, HttpRequest};

// fn main() {
//     let test = "test";
//     let test_function = || {
//         println!("test value is {}", test);
//         return test.to_owned() + &String::from(" case");
//     };
//
//     let outcome : String =  test_function();
//     println!("outcome value is {}", outcome);
// }

use std::{thread, time};
use std::thread::JoinHandle;

fn do_something(number : i8) -> i8 {
    println!("number is {}", number);
    let two_second = time::Duration::new(2, 0);
    thread::sleep(two_second);
    2
}

fn main() {
    let now = time::Instant::now();
    let one_thread    : JoinHandle<i8> = thread::spawn(|| do_something(1));
    let two_thread    : JoinHandle<i8> = thread::spawn(|| do_something(2));
    let three_thread  : JoinHandle<i8> = thread::spawn(|| do_something(3));

    let result_one = one_thread.join();
    let result_two = two_thread.join();
    let result_three = three_thread.join();

    println!("time elapsed: {:?}", now.elapsed());
    println!("result: {}", result_one.unwrap() + result_two.unwrap() + result_three.unwrap());
}

// async fn greet(req : HttpRequest) -> impl Responder{
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello, {}", name)
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         println!("function is firing");
//         let app  = App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet));
//         return app;
//     })
//         .bind("127.0.0.1:8000")?
//         .run()
//         .await
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let command: &String = &args[1];
//     let title: &String = &args[2];
//
//     let state: Map<String, Value> = read_file("/home/z1p1t/workspace/rust_web/state.json");
//     let status : String;
//     match &state.get(*&title) {
//         Some(result) => {
//             status = result.to_string().replace('\"', "");
//         },
//         None => {
//             status = String::from("pending");
//         }
//     }
//
//     let item = to_do_factory(&status, title).expect(&status);
//     process_input(item, command, &state);
// }
