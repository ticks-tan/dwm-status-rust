use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::fs::File;
use std::io::Read;

pub fn get_load_avg() -> ThreadsData {
    let mut buf = String::new();
    match File::open("/proc/loadavg") {
        Ok(mut file) => match file.read_to_string(&mut buf) {
            Ok(data) => data,
            _ => return ThreadsData::LoadAvg(String::from("")),
        },
        _ => return ThreadsData::LoadAvg(String::from("Error")),
    };
    let buf = buf.split_whitespace().collect::<Vec<_>>()[0];
    let data = format!("  {}  {}  {}", CONFIG.loadavg.icon, buf, CONFIG.seperator);
    ThreadsData::LoadAvg(data)
}
