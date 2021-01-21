use crate::config::CONFIG;
use std::fs::File;
use std::io::Read;

pub fn get_uptime() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    match File::open("/proc/uptime") {
        Ok(mut file) => file.read_to_string(&mut buf)?,
        _ => return Ok("cant find uptime file!".to_string()),
    };

    let buf: f32 = buf.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();

    let hour = buf.round() as u32 / 3600;
    let rem = buf as u32 - hour * 3600;
    let minutes = rem / 60;

    let uptime = if hour > 0 {
        format!("{}:{}", hour, minutes)
    } else {
        format!("{} min", minutes)
    };
    let result = format!("  {}  {}  {}", CONFIG.uptime.icon, uptime, CONFIG.seperator);
    Ok(result)
}
