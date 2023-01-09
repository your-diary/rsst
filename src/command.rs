use std::io::{BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_status: i32,
}

pub fn run(command: &str, args: &Vec<&str>, input: &str) -> CommandOutput {
    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = BufWriter::new(child.stdin.take().unwrap());
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let mut stderr = BufReader::new(child.stderr.take().unwrap());

    stdin.write_all(input.as_bytes()).unwrap();
    drop(stdin);

    let mut stdout_buf = String::new();
    stdout.read_to_string(&mut stdout_buf).unwrap();

    let mut stderr_buf = String::new();
    stderr.read_to_string(&mut stderr_buf).unwrap();

    let exit_status = child.wait().unwrap();

    CommandOutput {
        stdout: stdout_buf,
        stderr: stderr_buf,
        exit_status: exit_status.code().unwrap(),
    }
}
