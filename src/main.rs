mod utils;
use utils::{
    functions::load_configuration,
    manage_todos::{ show_todo_list, add_todo, reset_todo_tree, remove_todo },
};

use std::{ process::exit, env::args };
use json;

fn main() {
    let mut config: json::JsonValue = json::JsonValue::Null;
    load_configuration(&mut config);

    // Check if config `todos` exists
    if config["todos"].is_null() == true {
        println!("No todo list found in the configuration file.");
        exit(1);
    }

    // Working with arguments
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    if args.len() == 0 {
        show_todo_list(config);
        exit(0);
    }

    match args.remove(0).as_str() {
        "a" | "add" => add_todo(&config, args.join(" ").as_str()),
        "l" | "list" => show_todo_list(config),
        "r" | "remove" => remove_todo(config, args.get(0).expect("No index passed")),
        "R" | "reset" => reset_todo_tree(),
        _ => exit(1),
    }
}