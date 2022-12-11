use dirs::config_dir;
use std::fs;

pub struct Configuration {
    file_path: String,
}

impl Configuration {
    pub fn get_file_path(&self) -> &str {
        return &self.file_path;
    }
}

pub fn load_configuration(folder_name: &str) -> Configuration {
    let config_dir = format!("{}/{}", config_dir().unwrap().to_str().unwrap(), folder_name);
    fs::create_dir_all(&config_dir).expect("Could not create configuration directory.");

    let config_file = format!("{}/config.yml", config_dir);
    match fs::metadata(&config_file) {
        Ok(_o) => (),
        Err(_e) => fs::write(&config_file, "").expect("Could not create configuration directory."),
    }

    return Configuration { file_path: config_file };
}