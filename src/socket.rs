use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

#[derive(Debug)]
pub struct WorkspaceSocket {
    pub workspaces: HashMap<String, Vec<u64>>,
    pub focused: (String, u64),
    socket_path: PathBuf,
}

impl WorkspaceSocket {
    pub fn from(workspaces: HashMap<String, Vec<u64>>, focused: (String, u64)) -> Self {
        let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");
        let hyprland_instance_signature =
            env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE not set");

        let mut socket_path = PathBuf::from(xdg_runtime_dir);
        socket_path.push("hypr");
        socket_path.push(hyprland_instance_signature);
        socket_path.push(".socket2.sock");

        Self {
            workspaces,
            focused,
            socket_path,
        }
    }

    pub async fn listen(&mut self) -> io::Result<()> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        let reader = BufReader::new(stream);

        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            // change value of workspaces
            match line.as_str().split(">>").nth(0).unwrap() {
                "createworkspace" => {
                    let workspace_id = line
                        .as_str()
                        .split(">>")
                        .nth(1)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    self.workspaces
                        .entry(self.focused.0.clone())
                        .or_insert(Vec::new())
                        .push(workspace_id);
                }
                "workspace" => {
                    let workspace_id = line
                        .as_str()
                        .split(">>")
                        .nth(1)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    self.focused = (self.focused.0.clone(), workspace_id);
                }
                "destroyworkspace" => {
                    let workspace_id = line
                        .as_str()
                        .split(">>")
                        .nth(1)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    self.workspaces
                        .entry(self.focused.0.clone())
                        .or_insert(Vec::new())
                        .retain(|&x| x != workspace_id);
                }
                "focusedmon" => {
                    let monitor: (&str, &str) = line
                        .as_str()
                        .split(">>")
                        .nth(1)
                        .unwrap()
                        .split_once(",")
                        .unwrap();
                    let monitor = (monitor.0.to_string(), monitor.1.parse::<u64>().unwrap());
                    self.focused = monitor;
                }
                _ => {
                    continue;
                }
            }
            println!("{:#?}", self);
        }

        Ok(())
    }
}
