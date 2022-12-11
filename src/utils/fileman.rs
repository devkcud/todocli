use std::fs;

use yaml_rust::{ yaml::Array, YamlLoader, Yaml };

use super::configuration::Configuration;

pub fn load_yaml(config: &Configuration) -> Vec<Yaml> {
    let config_content = fs::read_to_string(config.get_file_path()).unwrap();
    let yaml = YamlLoader::load_from_str(&config_content);

    return yaml.expect("Could not load config file");
}

pub fn get_todos(config: &Configuration) -> Result<Array, &str> {
    let yaml = load_yaml(&config);

    if yaml.len() == 0 {
        return Err("Config file is empty");
    }

    let todos = &yaml[0]["todos"];

    if todos.is_badvalue() {
        return Err("Config file doesn't contain a todo array");
    }

    return Ok(todos.as_vec().unwrap().clone());
}