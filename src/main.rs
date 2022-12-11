mod utils;
use utils::configuration::load_configuration;

fn main() {
    let config = load_configuration("todocli");

    println!("{}", config.get_folder_path());
    println!("{}", config.get_file_path());
}