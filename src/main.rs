mod utils;
use utils::configuration::load_configuration;

use crate::utils::fileman::get_todos;

fn main() {
    let config = load_configuration("todocli");

    for todo in get_todos(&config).expect("got") {
        println!("{}", todo.as_str().unwrap());
    }
}