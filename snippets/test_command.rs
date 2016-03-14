
use std::process::Command;

fn main() {

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let output = output.stdout;

    println!("output: {:?}", String::from_utf8(output).unwrap());
}
