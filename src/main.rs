mod utils;
mod todoman;

use todoman::{ add_item, show_todo_list };
use crate::utils::configuration::load_configuration;

fn main() {
    let config = load_configuration("todocli");

    add_item(&config, "Hello", false);
    add_item(&config, "World!", false);
    show_todo_list(&config);
}