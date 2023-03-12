use serde::{Deserialize, Serialize};
use std::{
    env::{self, current_dir},
    fs::File,
    io::Read,
    path::Path,
    process::exit,
};

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
    if let Err(why) = config_file.read_to_string(&mut config_contents) {
        eprintln!("Failed to read: \"{}\"\n Error: {}", file_path_string, why);
        exit(1);
    }

    config_contents
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectDefinition {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HomeConfig {
    pub project: Vec<ProjectDefinition>,
}

fn get_home_config_path() -> String {
    let home_directory = env::var("HOME").unwrap();
    format!("{}/.pjconfig", home_directory)
}

pub fn get_home_config() -> HomeConfig {
    let home_file_path = get_home_config_path();
    let home_config_contents = read_config(home_file_path);
    let config = toml::from_str(home_config_contents.as_str()).unwrap();

    config
}

pub fn add_current_path_to_home_config(project_name: &str) {
    let current_config = get_home_config();

    let current_path = current_dir().unwrap();
    let current_path_str = current_path.into_os_string().into_string().unwrap();

    let mut filtered_projects = current_config
        .project
        .iter()
        .filter(|project| project.name != project_name)
        .cloned()
        .collect::<Vec<ProjectDefinition>>();

    filtered_projects.push(ProjectDefinition {
        name: project_name.to_string(),
        path: current_path_str,
    });

    let mut new_config = current_config;
    new_config.project = filtered_projects;

    let new_config = match toml::to_string_pretty(&new_config) {
        Err(why) => {
            eprintln!("Failed to serialize to toml. Error: {}", why);
            exit(1)
        }
        Ok(config) => config,
    };

    let home_file_path = get_home_config_path();
    if let Err(why) = std::fs::write(home_file_path, new_config) {
        eprintln!("Failed to serialize to toml. Error: {}", why);
        exit(1)
    }
}

#[derive(Deserialize, Debug)]
pub struct Window {
    pub name: String,
    pub folder: Option<String>,
    pub command: Option<String>,
    pub start: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Default {
    pub start: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SerialisedProjectConfig {
    #[serde(rename = "window")]
    pub windows: Vec<Window>,
    pub default: Option<Default>,
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

    let windows = match config_on_disk.default {
        Some(default) => config_on_disk
            .windows
            .into_iter()
            .map(|mut w| {
                if w.start.is_none() {
                    w.start = default.start.clone()
                }
                w
            })
            .collect(),
        None => config_on_disk.windows,
    };

    Project {
        windows,
        path: project_path,
        name: project.name.clone(),
    }
}
