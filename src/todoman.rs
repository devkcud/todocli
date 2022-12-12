use crate::utils::{ configuration::Configuration, fileman::get_todos };

use colored::Colorize;
use prettytable::{ Table, format, row };

pub fn show_todo_list(config: &Configuration) {
    let mut table = Table::new();
    table.set_format(format::FormatBuilder::new().padding(0, 3).build());

    let todos = get_todos(&config).expect("got");

    table.set_titles(row!["Index".blue().bold(), "Todo".blue().bold(), "Done".blue().bold()]);

    for (i, todo) in todos.iter().enumerate() {
        let index = (i + 1).to_string().yellow();
        let name = todo["name"].as_str().unwrap();
        let mut done = todo["done"].as_bool().unwrap().to_string();

        if done == "true" {
            done = done.green().to_string();
        } else {
            done = done.red().to_string();
        }

        table.add_row(row![index, name, done]);
    }

    table.add_row(row!["Size:".dimmed(), &todos.len().to_string().yellow()]);

    println!("{}", table.to_string().trim());
}