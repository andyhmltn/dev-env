use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use anyhow::Result;

pub enum RunnerMsg {
    Line(String),
    Done(Result<()>),
}

pub fn spawn_script(command: &str, repo_root: &Path) -> mpsc::Receiver<RunnerMsg> {
    let (tx, rx) = mpsc::channel();
    let command = command.to_string();
    let repo_root = repo_root.to_path_buf();

    thread::spawn(move || {
        let result = run_script(&command, &repo_root, &tx);
        let _ = tx.send(RunnerMsg::Done(result));
    });

    rx
}

fn run_script(command: &str, repo_root: &Path, tx: &mpsc::Sender<RunnerMsg>) -> Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .current_dir(repo_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let tx2 = tx.clone();
    let stderr_handle = thread::spawn(move || {
        if let Some(stderr) = stderr {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                let _ = tx2.send(RunnerMsg::Line(line));
            }
        }
    });

    if let Some(stdout) = stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            let _ = tx.send(RunnerMsg::Line(line));
        }
    }

    let _ = stderr_handle.join();

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("script exited with status {}", status)
    }
}
