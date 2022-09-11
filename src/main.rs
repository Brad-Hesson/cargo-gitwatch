use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Debug, Parser)]
struct CargoArgs {
    #[clap(subcommand)]
    gitwatch: GitWatchArgs,
}
#[derive(Debug, Subcommand)]
enum GitWatchArgs {
    Gitwatch {
        #[clap(long, short, multiple_values = true, allow_hyphen_values = true)]
        command: Option<Vec<String>>,
    },
}

fn main() -> Result<()> {
    let args = CargoArgs::parse();
    let GitWatchArgs::Gitwatch {
        command: command_arg,
    } = args.gitwatch;
    let mut command = match command_arg {
        Some(v) if v.len() > 0 => {
            let mut command = Command::new(v[0].as_str());
            command.args(v[1..].iter().map(|s| s.as_str()));
            command
        }
        _ => {
            let mut command = Command::new("cargo");
            command.args(["run"]);
            command
        }
    };
    let mut proc = command.spawn()?;
    while let None = proc.try_wait()? {
        if remote_updated()? {
            pull_changes()?;
            proc.kill()?;
            proc = command.spawn()?;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Ok(())
}

fn remote_updated() -> Result<bool> {
    Command::new("git")
        .args(["remote", "update"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    let stdout = Command::new("git")
        .args(["status"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?
        .wait_with_output()?
        .stdout;
    let output = std::str::from_utf8(stdout.as_slice())?;
    Ok(output.contains("Your branch is behind"))
}

fn pull_changes() -> Result<()> {
    Command::new("git")
        .args(["pull"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    Ok(())
}
