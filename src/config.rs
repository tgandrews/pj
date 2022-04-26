use serde::Deserialize;
use std::{env, fs::File, io::Read, path::Path, process::exit};

fn read_config(file_path_string: String) -> String {
    let file_path = Path::new(file_path_string.as_str());
    if !file_path.is_file() {
        eprintln!("Config is not a file: \"{}\"", file_path_string);
        exit(1)
    }

    let mut config_file = match File::open(file_path) {
        Err(why) => {
            eprintln!("{}", why);
            exit(1)
        }
        Ok(file) => file,
    };

    let mut config_contents = String::new();
    match config_file.read_to_string(&mut config_contents) {
        Err(why) => {
            eprintln!("Failed to read: \"{}\"\n Error: {}", file_path_string, why);
            exit(1)
        }
        Ok(_) => {}
    };

    return config_contents;
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct HomeConfig {
    pub project: Vec<Project>,
}

pub fn get_home_config() -> HomeConfig {
    let home_directory = env::var("HOME").unwrap();
    let home_file_path_string = format!("{}/.pjconfig", home_directory);
    let home_config_contents = read_config(home_file_path_string);
    let config = toml::from_str(home_config_contents.as_str()).unwrap();

    return config;
}

#[derive(Deserialize, Debug)]
pub struct Window {
    folder: String,
    command: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectConfig {
    project: Vec<Project>,
}

pub fn get_project_config(input_path: String) {
    let mut path = input_path.to_owned();
    if path.starts_with("~/") {
        let home_directory = env::var("HOME").unwrap();
        path.replace_range(0..1, home_directory.as_str());
    }
    println!("Project path: {}", path)
}
