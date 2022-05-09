use crate::projectron::{config, tmux};

mod projectron;

fn main() {
    let home_config = config::get_home_config();
    println!("{:?}", home_config);
    for project_definition in home_config.project {
        println!("Loading config for: {:?}", project_definition.name);
        let project = config::load_project(&project_definition);
        println!("{:?}", project);
        if !tmux::is_project_running(&project.name) {
            tmux::start(&project)
        }
        tmux::attach(&project)
    }
}
