mod utils;
mod todoman;

use crate::utils::configuration::load_configuration;
use crate::todoman::show_todo_list;

fn main() {
    let config = load_configuration("todocli");

    show_todo_list(&config);
}