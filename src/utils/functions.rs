use std::{ fs::{ read_to_string, write, create_dir }, path::Path };

use dirs::config_dir;

pub fn load_configuration(config: &mut json::JsonValue) {
    // Getting the config file/folder
    let config_dir = format!("{}/todocli", config_dir().unwrap().to_str().unwrap());
    let config_file = format!("{}/config.json", config_dir);

    (!Path::new(&config_dir).is_dir()).then(|| create_dir(&config_dir));
    (!Path::new(&config_file).is_file()).then(|| write(&config_file, "{ \"todos\": [] }"));

    // It's loading the configuration file into the `config` variable.
    *config = json::parse(&read_to_string(config_file).unwrap()).unwrap();
}

pub fn todo_to_vec(config: json::JsonValue) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    let mut todos = config["todos"].clone();

    // It's iterating over the `todos` array and pushing each item into the `list` vector.
    (0..todos.len()).for_each(|i| list.push(todos[i].take_string().unwrap()));

    return list;
}