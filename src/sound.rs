use crate::load_config::Settings;
use std::process::Command;

// TODO: what a horrible solution to get the sound, i dont like it
//       find another way you dumb fuck

pub fn get_sound(setting: &Settings) -> String {
    let cmd_content = Command::new("amixer")
        .arg("get")
        .arg("Master")
        .output()
        .unwrap();

    let mut vol: String = String::from_utf8_lossy(&cmd_content.stdout)
        .lines()
        .last()
        .expect("failed to get sound volume")
        .split('[')
        .collect::<Vec<&str>>()[1]
        .trim()
        .to_string();
    vol.remove(vol.len() - 1);

    format!("  {}  {}  {}", setting.volume.icon, vol, setting.seperator)
}
