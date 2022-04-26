use crate::config::get_project_config;

mod config;

fn main() {
    let home_config = config::get_home_config();
    println!("{:?}", home_config);
    for project in home_config.project {
        get_project_config(project.path);
    }
}
