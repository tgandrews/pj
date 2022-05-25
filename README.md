# Projectron (pj)

Command line tool for starting the necessary terminals for a project and shutting
them down. It is a wrapper for [tmux](https://github.com/tmux/tmux/wiki)

## Usage

### 1. Define a config to list the projects
Create a `~/.pjconfig` file in your home directory. This defines the projects that are available
```toml
[[project]]
name = "omnomnom" # name of the project
path = "~/src/omnomnom" # path to the root of the project

[[project]]
name = "omanyd"
path = "~/src/omanyd"
```

### 2. Create a config for the project
Create a `.pjconfig` file in the project directory
```toml
[[window]]
folder = "packages/frontend" # path to the sub project
name = "frontend" # name of the sub project
command = "yarn dev" # command to run the sub command

[[window]]
folder = "packages/brain"
name = "brain"
command = "yarn reticulate-splines"

[[window]]
folder = "packages/backend"
name = "backend"
command = "yarn test --watch"

[default]
start = "scripts/rc.sh" # command to run in all terminals useful
                        # for setting up project specific
                        # environment settings
```

### 3. Start the project
```bash
pj start ${project_name}
```
A tmux session is started with a window for each of them defined in the project
config. There are two panes with the command running in the left pane.

```
+---------+----------+
| command | empty    |
|         | terminal |
+---------+----------+
```

Leave the session with `Ctrl + b` and then `d`

### 4. Kill the project
```bash
pj end ${project_name}
```