use clap::{arg, command, Command};

use crate::projectron::{config, tmux};

mod projectron;

fn get_project(project_name: &str) -> config::Project {
    let home_config = config::get_home_config();
    for project_ref in home_config.project {
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
        tmux::kill(&project)
    }
}

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("end")
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
        .get_matches();

    match matches.subcommand() {
        Some(("end", sub_matches)) => end(sub_matches.value_of("PROJECT").unwrap()),
        Some(("start", sub_matches)) => start(sub_matches.value_of("PROJECT").unwrap()),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
