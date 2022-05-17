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
pub struct ProjectDefinition {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct HomeConfig {
    pub project: Vec<ProjectDefinition>,
}

pub fn get_home_config() -> HomeConfig {
    let home_directory = env::var("HOME").unwrap();
    let home_file_path = format!("{}/.pjconfig", home_directory);
    let home_config_contents = read_config(home_file_path);
    let config = toml::from_str(home_config_contents.as_str()).unwrap();

    return config;
}

#[derive(Deserialize, Debug)]
pub struct Window {
    pub folder: String,
    pub name: Option<String>,
    pub command: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SerialisedProjectConfig {
    #[serde(rename = "window")]
    pub windows: Vec<Window>,
}

#[derive(Debug)]
pub struct Project {
    pub windows: Vec<Window>,
    pub path: String,
    pub name: String,
}

pub fn load_project(project: &ProjectDefinition) -> Project {
    let mut project_path = project.path.clone();
    if project_path.starts_with("~/") {
        let home_directory = env::var("HOME").unwrap();
        project_path.replace_range(0..1, home_directory.as_str());
    }
    let config_path = format!("{}/.pjconfig", project_path);
    let config_contents = read_config(config_path);

    let config_on_disk: SerialisedProjectConfig = toml::from_str(config_contents.as_str()).unwrap();

    let config = Project {
        windows: config_on_disk.windows,
        path: project_path,
        name: project.name.clone(),
    };

    return config;
}
