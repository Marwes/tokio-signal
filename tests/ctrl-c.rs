// `Child::kill` terminates the application instead of sending the equivalent to SIGKILL on windows
#![cfg(all(unix, feature = "ctrl_c_example_test"))]

use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

#[test]
fn ctrl_c() {
    let ctrl_c_path = Path::new("target")
        .join("debug")
        .join("examples")
        .join("ctrl-c");
    let mut child = Command::new(ctrl_c_path)
        .stdout(Stdio::piped())
        .arg("ctrl_c")
        .spawn()
        .unwrap();

    // Wait for the first line so that the signal handler has been registered
    BufReader::new(child.stdout.as_mut().unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    for i in 0..5 {
        println!("Kill {}", i);
        child.kill().unwrap();
    }
    child.wait().unwrap();
}
