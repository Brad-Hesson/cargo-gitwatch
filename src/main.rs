use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Debug, Parser)]
struct MetaArgs {
    #[clap(subcommand)]
    gitwatch: Args,
}
#[derive(Debug, Subcommand)]
enum Args {
    Gitwatch {
        #[clap(long, short, multiple_values = true, allow_hyphen_values = true)]
        command: Option<Vec<String>>,
    },
}

fn main() {
    let args = MetaArgs::parse();
    let Args::Gitwatch {
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
    let mut proc = command.spawn().unwrap();
    loop {
        if proc.try_wait().unwrap().is_some() {
            break;
        }
        if remote_updated() {
            pull_changes();
            proc.kill().unwrap();
            proc = command.spawn().unwrap();
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn remote_updated() -> bool {
    Command::new("git")
        .args(["remote", "update"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    let stdout = Command::new("git")
        .args(["status"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
        .stdout;
    let output = std::str::from_utf8(stdout.as_slice()).unwrap();
    output.contains("Your branch is behind")
}

fn pull_changes() {
    Command::new("git")
        .args(["pull"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
