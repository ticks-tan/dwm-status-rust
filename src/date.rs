use std::process::Command;

pub fn fmt_date(ft: &str) -> String {
    let cmd = format!("date +\"{}\"", ft);
    let cmd = Command::new("sh").arg("-c").args(&[cmd]).output().unwrap();
    let result = String::from_utf8_lossy(&cmd.stdout)
        .to_string()
        .trim()
        .to_string();
    format!("    {}  │", result)
}
