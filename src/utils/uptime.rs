use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::fs::read_to_string;

pub fn get_uptime() -> ThreadsData {
    let buf = match read_to_string("/proc/uptime") {
        Ok(data) => data,
        _ => return ThreadsData::Uptime("cant find uptime file!".to_string()),
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
    ThreadsData::Uptime(result)
}
