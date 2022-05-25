use std::process::{Command, ExitStatus};

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
    run_tmux(vec![
        "new-window",
        "-t",
        identity.as_str(),
        "-n",
        &window.name.as_str(),
    ])
}

fn split_window(identity: &String, window: &Window, project: &Project) {
    run_tmux(vec!["split-window", "-h", "-t", identity.as_str()]);
    let window_path = match &window.folder {
        Some(folder) => format!("{}/{}", project.path, folder),
        None => project.path.clone(),
    };
    let command = match &window.command {
        Some(cmd) => cmd.clone(),
        None => format!("echo \"{}\"", window.name),
    };

    let move_to_window_path = format!("cd {}", window_path);
    let mut basic_actions = vec![move_to_window_path, "clear".to_string()];
    if window.start.is_some() {
        basic_actions.push(format!(
            "source {}/{}",
            project.path,
            window.start.clone().unwrap().to_string()
        ))
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
        // Prefix commands with a space to ensure they aren't added to the history
        format!(" {}", statement).as_str(),
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
    let output = run_tmux_with_output(vec!["ls"]);

    if output.status.success() {
        let expected_string = format!("{}: ", name);
        return output.stdout.contains(&expected_string);
    }

    // No tmux sessions running
    if output.stderr.starts_with("no server running on") {
        return false;
    }
    if output
        .stderr
        .starts_with("error connecting to /private/tmp/tmux")
    {
        return false;
    }
    panic!(
        "Unknown tmux failure!\nStatus: {}\nStdErr: {}",
        output
            .status
            .code()
            .expect("Unable to unwrap status code after tmux ls error"),
        output.stderr
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

struct TmuxResult {
    status: ExitStatus,
    stderr: String,
    stdout: String,
}

fn run_tmux_with_output(args: Vec<&str>) -> TmuxResult {
    let output = Command::new("tmux")
        .args(&args)
        .output()
        .expect("failed to execute tmux");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    let result = TmuxResult {
        status: output.status,
        stderr,
        stdout,
    };

    return result;
}

struct RunningPane<'a> {
    project: &'a str,
    pane_id: &'a str,
    // running_command: &'a str,
}

fn kill_running_panes_for_project(project: &Project) {
    let all_running_panes_output = run_tmux_with_output(vec![
        "list-panes",
        "-a",
        "-F",
        "#{pane_id},#{pane_current_command},#{session_name}",
    ]);
    if !all_running_panes_output.status.success() {
        panic!(
            "Unable to list tmux panes:\n{}",
            all_running_panes_output.stderr
        );
    }
    let running_panes_for_project: Vec<RunningPane> = all_running_panes_output
        .stdout
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let values: Vec<&str> = line.split(",").collect();
            return RunningPane {
                pane_id: values[0],
                // running_command: values[1],
                project: values[2],
            };
        })
        .filter(|pane| pane.project == project.name)
        .collect();

    for pane in running_panes_for_project {
        run_tmux(vec!["send-keys", "-t", pane.pane_id, "C-c"])
    }
}

pub fn kill(project: &Project) {
    kill_running_panes_for_project(project);

    run_tmux(vec!["kill-session", "-t", project.name.as_str()]);
}
