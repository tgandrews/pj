use clap::{arg, command, Command};
use pj::config::ProjectDefinition;

use crate::pj::{config, tmux};

mod pj;

fn get_project_defintions() -> Vec<ProjectDefinition> {
    let home_config = config::get_home_config();
    home_config.project
}

fn get_project(project_name: &str) -> config::Project {
    let projects = get_project_defintions();
    for project_ref in projects {
        if project_ref.name == project_name {
            return config::load_project(&project_ref);
        }
    }
    panic!("No project called: {}", project_name)
}

fn start(project_name: &str) {
    let project = get_project(project_name);
    if !tmux::is_project_running(&project.name) {
        tmux::start(&project)
    }
    tmux::attach(&project);
}

fn end(project_name: &str) {
    let project = get_project(project_name);
    if tmux::is_project_running(&project.name) {
        tmux::kill(&project);
        println!("Successfully killed: {}", project_name)
    } else {
        println!("{} is not running", project.name)
    }
}

fn list() {
    let running_projects = get_project_defintions()
        .iter()
        .filter(|p| tmux::is_project_running(&p.name))
        .map(|p| p.name.clone())
        .collect::<Vec<String>>();

    if running_projects.is_empty() {
        println!("No projects are running");
        return;
    }

    println!("{}", running_projects.join("\n"));
}

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("end")
                .alias("stop")
                .about("End an existing session")
                .arg_required_else_help(true)
                .arg(arg!([PROJECT])),
        )
        .subcommand(
            Command::new("start")
                .about("Start or join an session")
                .arg_required_else_help(true)
                .arg(arg!([PROJECT])),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("Get the running projects"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", sub_matches)) => start(sub_matches.value_of("PROJECT").unwrap()),
        Some(("end", sub_matches)) => end(sub_matches.value_of("PROJECT").unwrap()),
        Some(("list", _)) => list(),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
