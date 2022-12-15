use std::fs;

use crate::utils::{ configuration::Configuration, fileman::get_todos };

use colored::Colorize;
use prettytable::{ Table, format, row };
use yaml_rust::YamlLoader;

pub fn show_todo_list(config: &Configuration) {
    let mut table = Table::new();
    table.set_format(format::FormatBuilder::new().padding(0, 3).build());

    let todos = get_todos(&config).expect("got");

    table.set_titles(row!["Index".blue().bold(), "Todo".blue().bold(), "Done".blue().bold()]);

    for (i, todo) in todos.iter().enumerate() {
        let index = (i + 1).to_string().yellow();
        let name = todo["name"].as_str().unwrap();
        let mut done = todo["done"].as_bool().unwrap().to_string();

        done = (done == "true")
            .then(|| done.green().to_string())
            .unwrap_or_else(|| done.red().to_string());

        table.add_row(row![index, name, done]);
    }

    table.add_row(row!["Size:".dimmed(), &todos.len().to_string().yellow()]);

    println!("{}", table.to_string().trim());
}

pub fn add_item(config: &Configuration, name: &str, done: bool) {
    let mut todos = get_todos(&config).expect("got");
    let new_todo_string = format!("{{ name: \"{}\", done: {done} }}", name.replace("\"", ""));

    todos.push(YamlLoader::load_from_str(&new_todo_string).unwrap()[0].clone());

    let mut out_str = "# Generated automatically don't manually change it.\ntodos: [".to_string();

    for todo in todos {
        let r_name = todo["name"].as_str().unwrap();
        let r_done = todo["done"].as_bool().unwrap();

        let f = format!("{}{{name: \"{}\",done: {}}},", out_str, r_name, r_done);
        out_str = f;
    }

    out_str = out_str + " ]";
    fs::write(config.get_file_path(), &out_str).expect("Failed to add the new todo to the list");
}

pub fn remove_all(config: &Configuration) {
    fs::write(config.get_file_path(), "todos: []").expect("Failed to write to file")
}