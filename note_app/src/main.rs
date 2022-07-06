mod state;

use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("{:?}", state);
    state.insert(title.to_string(), json!(status));
    write_to_file("./state.json", &mut state);
}

// mod to_do;

// use to_do::structs::traits::create::Create;
// use to_do::to_do_factory;
// use to_do::ItemTypes;

// fn main() {
//     let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");
//     match to_do_item.unwrap() {
//         ItemTypes::Pending(item) => item.create(&item.super_struct.title),
//         ItemTypes::Done(item) => println!(
//             "it's a done item with the title: {}",
//             item.super_struct.title
//         ),
//     }
// }
