use std::process::Command;

pub fn os_command_example_1() {
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo aye there from Window!"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .args(["-c", "echo aye there from Linux!"])
            .output()
            .expect("Failed to execute command")
    };
    println!("\n\n");

    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte response");
    println!("{}", cmd_output);

    println!("\n\n");
}

fn os_command_example_2() {
    println!("\n\n");
    let mut cmd_root = Command::new("ls");

    cmd_root.status().expect("Failed to execute list command.");
    println!("\n\n");

    cmd_root
        .current_dir("src")
        .status()
        .expect("Failed to execute list command.");
    println!("\n\n");
}