extern crate dirs;
extern crate yaml_rust;
extern crate term_grid;
extern crate colored;

use std::{path::Path, fs::{self, File}, io::Read};

use dirs::config_dir;
use yaml_rust::{YamlLoader};
use term_grid::{GridOptions, Grid, Direction, Cell};
use colored::Colorize;

fn main() {
    // Getting the config
    let config_dir = config_dir().unwrap();
    let config_dir = config_dir.to_str().unwrap();

    let config_dir = format!("{}/todocli", config_dir.replace("\\", "/"));
    let config_file = format!("{}/config.yml", config_dir);

    if Path::new(&config_dir).is_dir()   == false { fs::create_dir_all(&config_dir).expect("Error creating config folder."); }
    if Path::new(&config_file).is_file() == false { fs::write(&config_file, "todos: []").expect("Error creating config file."); }

    let mut file_data = String::new();
    File::open(config_file).unwrap().read_to_string(&mut file_data).expect("Error reading config file.");

    let config = &YamlLoader::load_from_str(&file_data).unwrap();

    if config.len() == 0 { return; }
    let config = &config[0];

    let mut output_grid = Grid::new(GridOptions { direction: Direction::LeftToRight, filling: term_grid::Filling::Spaces(1) });

    // Iterating over the todos in the config file and adding them to the output_grid.
    for (i, todo) in config["todos"].as_vec().unwrap().iter().enumerate() {
        output_grid.add(Cell::from(i.to_string().yellow().to_string()));
        output_grid.add(Cell::from(todo.as_str().unwrap().blue().to_string()));
    }

    println!("{}", output_grid.fit_into_columns(2).to_string().trim());
    println!("Quantity of items: {}", &config["todos"].as_vec().unwrap().len().to_string().yellow().to_string());
}
