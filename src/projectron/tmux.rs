use std::process::Command;

use super::config::{Project, Window};

pub fn start(project: &Project) {
    run_tmux(vec![
        "new-session",
        "-d",
        "-s",
        project.name.as_str(),
        "-t",
        project.name.as_str(),
    ]);

    for (i, window) in project.windows.iter().enumerate() {
        let window_identity = format!("{}:{}", project.name, i + 1);

        start_window(&window_identity, &window);
        split_window(&window_identity, &window, &project);
    }
}

fn start_window(identity: &String, window: &Window) {
    let window_name = match &window.name {
        Some(name) => name,
        None => &window.folder,
    };

    run_tmux(vec![
        "new-window",
        "-t",
        identity.as_str(),
        "-n",
        window_name.as_str(),
    ])
}

fn split_window(identity: &String, window: &Window, project: &Project) {
    run_tmux(vec!["split-window", "-h", "-t", identity.as_str()]);
    let window_path = format!("{}/{}", project.path, window.folder);
    let command = match &window.command {
        Some(cmd) => cmd.clone(),
        None => format!("echo \"{}\"", window.folder),
    };

    let move_to_window_path = format!("cd {}", window_path);
    let mut basic_actions = vec![move_to_window_path, "clear".to_string()];
    match &window.start {
        Some(start) => basic_actions.push(format!("source {}/{}", project.path, start.to_string())),
        None => {}
    }

    let main_window_additional_commands = vec![command];

    run_command_in(
        format!("{}.0", identity),
        basic_actions
            .clone()
            .into_iter()
            .chain(main_window_additional_commands.into_iter())
            .collect(),
    );

    run_command_in(format!("{}.1", identity), basic_actions)
}

fn run_command_in(identity: String, commands: Vec<String>) {
    let statement = commands.join(" && ");

    run_tmux(vec![
        "send-keys",
        "-t",
        identity.as_str(),
        statement.as_str(),
        "Enter",
    ]);
}

fn run_tmux(args: Vec<&str>) {
    let status = Command::new("tmux")
        .args(&args)
        .status()
        .expect(format!("Failed to execute: tmux").as_str());
    if !status.success() {
        panic!(
            "Failed to execute: tmux {:?} (Status: {})",
            args,
            status.code().unwrap()
        )
    }
}

pub fn is_project_running(name: &String) -> bool {
    let output = Command::new("tmux")
        .arg("ls")
        .output()
        .expect("failed to execute tmux ls");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if output.status.success() {
        let expected_string = format!("{}: ", name);
        return stdout.contains(&expected_string);
    }

    // No tmux sessions running
    if stderr.starts_with("no server running on") {
        return false;
    }
    if stderr.starts_with("error connecting to /private/tmp/tmux") {
        return false;
    }
    panic!(
        "Unknown tmux failure!\nStatus: {}\nStdErr: {}",
        output
            .status
            .code()
            .expect("Unable to unwrap status code after tmux ls error"),
        stderr
    )
}

pub fn attach(project: &Project) {
    let mut tmux = Command::new("tmux")
        .args(vec!["-2", "attach-session", "-t", project.name.as_str()])
        .spawn()
        .expect("failed to spawn tmux");

    let exit_status = tmux.wait().expect("Failed to wait for tmux");

    assert!(exit_status.success());
}

pub fn kill(project: &Project) {
    run_tmux(vec!["kill-session", "-t", project.name.as_str()]);
}
