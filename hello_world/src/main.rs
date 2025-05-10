mod sayhello;

use sayhello::say_hello;
use serde::{Deserialize, Serialize};
use std::thread::spawn;

fn print_something() {
    println!("Hello, world!22");
}

fn main() {
    println!("Hello, world!");
    print_something();
    say_hello();

    let handle = spawn(|| {
        println!("Hello from a thread!");
        #[derive(Serialize, Deserialize, Debug)]
        struct MyStruct {
            name: String,
            age: u32,
        }

        let my_struct = MyStruct {
            name: String::from("Alice"),
            age: 30,
        };

        let serialized = serde_json::to_string(&my_struct);
        match serialized {
            Ok(json) => println!("Serialized JSON: {}", json),
            Err(e) => println!("Error serializing: {}", e),
        }
    });

    handle.join().unwrap();
}
