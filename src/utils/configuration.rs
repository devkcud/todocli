use dirs::config_dir;
use std::fs;

pub struct Configuration {
    folder_path: String,
    file_path: String,
}

impl Configuration {
    pub fn get_file(&self) -> &str {
        &self.file_path
    }

    pub fn get_folder(&self) -> &str {
        &self.folder_path
    }
}

pub fn load_configuration() -> Configuration {
    let config_dir = format!("{}/todocli", config_dir().unwrap().to_str().unwrap());
    fs::create_dir_all(&config_dir).expect("Could not create configuration directory.");

    let config_file = format!("{}/config.yml", config_dir);
    match fs::metadata(&config_file) {
        Ok(_o) => (),
        Err(_e) => fs::write(&config_file, "").expect("Could not create configuration directory."),
    }

    Configuration { folder_path: config_dir, file_path: config_file }
}