use std::fs::write;

use dirs::config_dir;
use term_grid::{ Grid, GridOptions, Cell };

use crate::utils::functions::todo_to_vec;
use colored::Colorize;
use json::object;

pub fn show_todo_list(config: json::JsonValue) {
    let mut grid = Grid::new(GridOptions {
        direction: term_grid::Direction::LeftToRight,
        filling: term_grid::Filling::Spaces(1),
    });

    let todos = todo_to_vec(config);

    grid.add(Cell::from("Index".yellow().bold().to_string()));
    grid.add(Cell::from("Todo".blue().bold().to_string()));

    for (i, todo) in todos.iter().enumerate() {
        grid.add(Cell::from(i.to_string().yellow().to_string()));
        grid.add(Cell::from(todo.to_string().blue().to_string()));
    }

    if todos.len() > 0 {
        println!("{}\n", grid.fit_into_columns(2).to_string().trim());
    }

    println!("Size: {}", todos.len().to_string().yellow());
}

pub fn add_todo(config: &json::JsonValue, what_to_do: &str) {
    let mut todos = todo_to_vec(config.to_owned());

    if what_to_do.trim().is_empty() {
        return;
    }

    todos.push(what_to_do.to_string());

    let obj = object! { "todos": todos };
    let cfg_file = format!("{}/todocli/config.json", config_dir().unwrap().to_str().unwrap());
    write(cfg_file, obj.to_string()).unwrap();
}

pub fn remove_todo(config: json::JsonValue, index: &str) {
    let mut todos = todo_to_vec(config);

    let index: usize = index
        .parse::<i32>()
        .unwrap()
        .max(0)
        .min(i32::try_from(todos.len() - 1).unwrap())
        .try_into()
        .expect("index out of bounds");

    todos.get(index).expect("Out of bounds");

    todos.remove(index);

    let obj = object! { "todos": todos };
    let cfg_file = format!("{}/todocli/config.json", config_dir().unwrap().to_str().unwrap());
    write(cfg_file, obj.to_string()).unwrap();
}

pub fn reset_todo_tree() {
    let v: Vec<String> = vec![];
    let obj = object! { "todos": v };
    let cfg_file = format!("{}/todocli/config.json", config_dir().unwrap().to_str().unwrap());
    write(cfg_file, obj.to_string()).unwrap();
}