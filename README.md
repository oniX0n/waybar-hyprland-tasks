# waybar-hyprland-tasks

Quick taskbar like thingy i wrote in rust. Compiles two binaries one generates the text for a custom waybar module, the other one signals to update it.
It uses the hyprland sockets and a unix signal.

To get it to work you need to setup a custom waybar module wich executes the tasks binary with `tasks -w`. It need to update on signal 8.
Im using signal RTMIN+8 you can change it though in the rust source and the waybar module.
Also you need to start the signaler binary, i use hyprland `exec-once=signaler`
Both binaries need to be in your path.

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
```

This is bare minimum right now, it only displays the name of the application, no icon. Also the tasks are not ordered yet.
