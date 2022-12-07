use std::{ path::Path, fs::{ create_dir, read_to_string, write }, process::exit };

use dirs::config_dir;
use json;
use term_grid::{ Grid, GridOptions, Cell };
use colored::Colorize;

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

fn load_configuration(config: &mut json::JsonValue) {
    // Getting the config file/folder
    let config_dir = format!("{}/todocli", config_dir().unwrap().to_str().unwrap());
    let config_file = format!("{}/config.json", config_dir);

    (!Path::new(&config_dir).is_dir()).then(|| create_dir(&config_dir));
    (Path::new(&config_file).is_file() == false).then(|| write(&config_file, "{ \"todos\": [] }"));
    // It's loading the configuration file into the `config` variable.
    *config = json::parse(&read_to_string(config_file).unwrap()).unwrap();
}

fn todo_to_vec(config: json::JsonValue) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    let mut todos = config["todos"].clone();

    // It's iterating over the `todos` array and pushing each item into the `list` vector.
    (0..todos.len()).for_each(|i| list.push(todos[i].take_string().unwrap()));

    return list;
}

fn show_todo_list(config: json::JsonValue) {
    let mut grid = Grid::new(GridOptions {
        direction: term_grid::Direction::LeftToRight,
        filling: term_grid::Filling::Spaces(1),
    });

    let todos = todo_to_vec(config);

    for (i, todo) in todos.iter().enumerate() {
        grid.add(Cell::from(i.to_string().yellow().to_string()));
        grid.add(Cell::from(todo.to_string().blue().to_string()));
    }

    if todos.len() > 0 {
        println!("{}\n", grid.fit_into_columns(2).to_string().trim());
    }

    println!("Size: {}", todos.len().to_string().yellow());
}