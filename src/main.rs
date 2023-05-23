use std::os::unix::process::CommandExt;

use swayipc::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sway = Connection::new().unwrap();

    let outputs = sway.get_tree()?.nodes;

    let mut workspaces: Vec<_> = outputs.as_slice().iter()
        .flat_map(|outputs| outputs.nodes.as_slice())
        .filter(|workspace| workspace.name.as_deref() != Some("__i3_scratch"))
        .collect();

    // Try spawn on existed empty workspace
    workspaces.sort_by(|w1, w2| w1.num.cmp(&w2.num));
    for ws in &workspaces {
        if ws.nodes.is_empty() {
            return spawn_command(ws.num.unwrap(), &mut sway);
        }
    }

    // Try spawn on a new workspace
    let mut i = 1;
    for _ in 0..9 {
        if workspaces.iter().any(|ws| ws.num.unwrap() == i) {
            i = i + 1;
            continue
        }
        return spawn_command(i, &mut sway);
    }

    // Try spawn on a workspace with the least number of windows
    workspaces.sort_by(|w1, w2| w1.nodes.len().cmp(&w2.nodes.len()));
    spawn_command(workspaces.first().unwrap().num.unwrap(), &mut sway)
}


fn spawn_command(ws_num: i32, sway: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    sway.run_command(&format!("workspace number {}", ws_num))?;

    if std::env::args().len() == 1 {
        return Ok(())
    }

    let mut cmd = std::process::Command::new(std::env::args().nth(1).unwrap());
    if std::env::args().len() > 2 {
        cmd.args(std::env::args().skip(2));
    }

    let err = cmd.exec();
    Err(err.into())
}
