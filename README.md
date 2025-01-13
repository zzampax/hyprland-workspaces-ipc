# Hyprland Workspaces IPC Script
## Introduction
Real time utility that tracks every workspace currently in use in [Hyprland](https://github.com/hyprwm/Hyprland) filtered by monitor output, example:
```json
{
    "workspaces": {
        "HDMI-3": [
            2,
        ],
        "DP-1": [
            1,
        ],
    },
    "focused": (
        "DP-1",
        1,
    ),
    "socket_path": "[...]/.socket2.sock",
}
```
The script relies on `tokio` for handling asynchronous calls to the **UNIX** socket provided by Hyprland and `serde_json` for handling **JSON** encoded output from `hyprctl`.
## How it Works
On execution, the script fetches the current workspaces (filtered by monitor) as long with the focused monitor+workspace pair with the `hyprctl commands`:
```bash
# For workspace list
hyprctl workspaces -j # -j arg for JSON output
# For currently focused workspace
hyprctl activeworkspace -j
```
Adapted for script execution in Rust (using `std::process::{Command, Output}` structs):
```rust
// For workspace list
let output: Output = Command::new("hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .expect("Failed to execute command");
// Same concept for currently focused workspace
// ...
```
The script then proceeds to listen for event calls on the UNIX socket such as:
- `createworkspace`
- `workspace`
- `destroyworkspace`
- `focusedmon`

> Keep in mind than no `v2` is considered
## Why?
This project aims to provide a lightweight solution to handle live workspaces information for a taskbar (Wayland, Hyprland).
## License
[GNU Affero General Public License v3.0](LICENSE)
