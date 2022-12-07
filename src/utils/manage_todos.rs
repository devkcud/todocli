use term_grid::{ Grid, GridOptions, Cell };

use crate::utils::functions::todo_to_vec;
use colored::Colorize;

pub fn show_todo_list(config: json::JsonValue) {
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