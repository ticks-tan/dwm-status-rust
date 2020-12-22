use crate::load_config::Settings;
use std::process::Command;

pub fn disk_free(setting: &Settings) -> String {
    let cmd = Command::new("sh")
        .arg("-c")
        .args(&["df -h"])
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&cmd.stdout);
    let mut disk_used = String::new();
    for line in output.lines() {
        if line.ends_with('/') {
            let splited = line.split_whitespace().collect::<Vec<&str>>();
            disk_used = splited[2].to_string();
            break;
        }
    }
    format!(
        "  {}  {}  {}",
        setting.disk.icon, disk_used, setting.seperator
    )
}
