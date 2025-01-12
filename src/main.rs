use serde_json::Value;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::str::from_utf8;

mod socket;
use crate::socket::WorkspaceSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output: Output = Command::new("hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .expect("Failed to execute command");
    let output: &str = from_utf8(&output.stdout)?;

    let data: Vec<Value> = serde_json::from_str(output)?;
    let mut workspaces: HashMap<String, Vec<u64>> = HashMap::new();

    for item in data {
        if let (Some(monitor), Some(id)) = (item.get("monitor"), item.get("id")) {
            let monitor: String = monitor.as_str().unwrap().to_string();
            let id: u64 = id.as_u64().unwrap();
            workspaces.entry(monitor).or_insert(Vec::new()).push(id);
        }
    }

    let output: Output = Command::new("hyprctl")
        .arg("activeworkspace")
        .arg("-j")
        .output()
        .expect("Failed to execute command");
    let output: &str = from_utf8(&output.stdout)?;
    let data: Value = serde_json::from_str(output)?;

    let focused_monitor: String = data.get("monitor").unwrap().as_str().unwrap().to_string();
    let focused_workspace: u64 = data.get("id").unwrap().as_u64().unwrap();
    println!("{:#?} Focus: {focused_monitor}", workspaces);

    let mut socket = WorkspaceSocket::from(workspaces, (focused_monitor, focused_workspace));
    socket.listen().await?;

    Ok(())
}
