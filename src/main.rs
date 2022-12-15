mod utils;
mod todoman;

use todoman::{ add_item, show_todo_list, remove_all, toggle_done };
use crate::utils::configuration::load_configuration;

fn main() {
    let config = load_configuration("todocli");

    add_item(&config, "Hello", false);
    toggle_done(&config, 1);
    add_item(&config, "World!", true);
    toggle_done(&config, 2);
    show_todo_list(&config);
    remove_all(&config);
}