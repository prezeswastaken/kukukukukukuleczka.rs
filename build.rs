use std::process::Command;

fn main() {
    let output = Command::new("bash")
        .arg("-c")
        .arg("source .env && echo $OPENAI_API_KEY")
        .output()
        .expect("Failed to execute command");

    if !output.stderr.is_empty() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    let variable_value = String::from_utf8(output.stdout).unwrap().trim().to_string();
    println!("cargo:rustc-env=OPENAI_API_KEY={}", variable_value);
}
