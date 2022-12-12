use dirs::config_dir;
use std::fs::{ create_dir_all, metadata, write };

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
    create_dir_all(&config_dir).expect("Could not create configuration directory.");

    let config_file = format!("{}/config.yml", config_dir);
    match metadata(&config_file) {
        Ok(_o) => (),
        Err(_e) =>
            write(
                &config_file,
                "# Generated automatically don't manually change it.\ntodos: []"
            ).expect("Could not create configuration directory."),
    }

    return Configuration { file_path: config_file };
}