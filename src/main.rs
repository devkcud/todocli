use std::{path::Path, fs, process::exit};

use dirs::config_dir;
use yaml_rust::{YamlLoader, Yaml};
use term_grid::{GridOptions, Grid, Direction, Cell};
use colored::Colorize;

fn main() {
    let mut config: Vec<Yaml> = vec![];
    load_configuration(&mut config);
    let config = &config[0];

    // Check if config `todos` exists
    if config["todos"].is_badvalue() == true {
        println!("The configuration file's to-do list section is missing.");
        exit(1);
    }

    show_todo_list(&config);
}

fn load_configuration(config: &mut Vec<Yaml>) {
    // Getting the config file/folder
    let config_dir = format!("{}/todocli", config_dir().unwrap().to_str().unwrap().replace("\\", "/"));
    let config_file = format!("{}/config.yml", config_dir);

    if Path::new(&config_dir).is_dir()   == false { fs::create_dir(&config_dir).expect("Error creating config folder."); }
    if Path::new(&config_file).is_file() == false { fs::write(&config_file, "todos: []").expect("Error creating config file."); }

    // It's loading the configuration file into the `config` variable.
    *config = YamlLoader::load_from_str(&fs::read_to_string(config_file).unwrap()).unwrap();
}

fn show_todo_list(config: &Yaml) {
    let mut output_grid = Grid::new(GridOptions { direction: Direction::LeftToRight, filling: term_grid::Filling::Spaces(1) });

    // Iterating over the todos in the config file and adding them to the output_grid.
    for (i, todo) in config["todos"].as_vec().unwrap().iter().enumerate() {
        output_grid.add(Cell::from(i.to_string().yellow().to_string()));
        output_grid.add(Cell::from(todo.as_str().unwrap().blue().to_string()));
    }

    let todo_size = config["todos"].as_vec().unwrap().len();

    if todo_size == 0 { return; }

    println!("{}", output_grid.fit_into_columns(2).to_string().trim());
    println!("\nQuantity of items: {}", todo_size.to_string().yellow().to_string());
}
