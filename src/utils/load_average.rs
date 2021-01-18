use crate::types::Config;
use std::fs::File;
use std::io::Read;

pub fn get_load_avg(config: &Config) -> String {
    let mut buf = String::new();
    match File::open("/proc/loadavg") {
        Ok(mut file) => match file.read_to_string(&mut buf) {
            Ok(data) => data,
            _ => return String::from(""),
        },
        _ => return String::from("Error"),
    };
    let buf = buf.split_whitespace().collect::<Vec<_>>()[0];
    format!("  {}  {}  {}", config.loadavg.icon, buf, config.seperator)
}
