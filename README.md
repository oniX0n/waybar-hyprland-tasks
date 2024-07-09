# waybar-hyprland-tasks
Taskbar like thingy i wrote in rust with support for workspaces. For waybar and hyprland. Displays currently open windows in hyprland workspace.

## Setup
Compiles two binaries; one generates the text for a custom waybar module, the other signals to update it.
It uses the hyprland sockets and a unix signal.

1. Compile both binaries with `cargo build -r` and place the two, resulting binaries in `target/release` somewhere in your path
2. Setup a custom waybar module. More information in the waybar wiki. It needs to execute `tasks -w` and update on signal 8.
   Snippet of my waybar config:
```
...
"modules-center": ["custom/tasks"],
...
"custom/tasks": {
  "exec": "tasks -w",
  "format": "{}",
  "interval": "once",
  "signal": 8
},
...
```
3. Start the signaler on login: I use `exec-once=signaler` in my hyrpland config

## Disclaimer

This is bare minimum right now, it only displays the name of the application, no icon. Also the tasks are not ordered yet.

## TODO

- [ ] Configuration framework
- [ ] Order tasks
- [ ] More features: grouping tasks with same name, hint fullscreen task, ...
- [ ] (Icons)
