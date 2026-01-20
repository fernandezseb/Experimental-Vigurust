fn main() {
    use std::process::Command;

    // TODO: Improve java compilation in build script
    let mut echo_hello = Command::new("javac");
    echo_hello.args(["-source", "1.8", "-target", "1.8"]).arg("Main.java");
    let output = echo_hello.output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}