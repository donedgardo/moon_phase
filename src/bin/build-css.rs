use std::process::Command;

fn main() {
    let output = Command::new("npx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("./src/input.css")
        .arg("-o")
        .arg("./static/output.css")
        .output()
        .expect("failed to execute process");
    println!(
        "CSS Build Finished{}{}{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
