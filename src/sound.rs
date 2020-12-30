use crate::load_config::Settings;
use std::process::Command;

// TODO: what a horrible solution to get the sound, i dont like it
//       find another way you dumb fuck

pub fn get_sound(setting: &Settings) -> String {
    let cmd_content = Command::new("amixer")
        .args(&["-D", "pulse", "get", "Master"])
        .output()
        .expect("Make sure that you have alsa-utils installed on your system");

    let vol: String = String::from_utf8_lossy(&cmd_content.stdout)
        .lines()
        .last()
        .expect("failed to get sound volume")
        .split('[')
        .collect::<Vec<&str>>()[1]
        .split(']')
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_string();

    format!("  {}  {}  {}", setting.volume.icon, vol, setting.seperator)
}
