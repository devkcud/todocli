mod utils;
mod todoman;

use todoman::{ add_item, show_todo_list, remove_all };
use crate::utils::configuration::load_configuration;

fn main() {
    let config = load_configuration("todocli");

    add_item(&config, "Hello", false);
    add_item(&config, "World!", true);
    show_todo_list(&config);
    remove_all(&config);
}