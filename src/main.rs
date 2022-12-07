mod utils;
use utils::{ functions::{ load_configuration }, manage_todos::show_todo_list };

use std::process::exit;
use json;

fn main() {
    let mut config: json::JsonValue = json::JsonValue::Null;
    load_configuration(&mut config);

    // Check if config `todos` exists
    if config["todos"].is_null() == true {
        println!("No todo list found in the configuration file.");
        exit(1);
    }

    show_todo_list(config);
}