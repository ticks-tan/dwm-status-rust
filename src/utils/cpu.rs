use crate::types::Config;
use std::fs::File;
use std::io::Read;

// getting cpu temperature
pub fn get_cpu_temp(config: &Config) -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/sys/class/thermal/thermal_zone0/temp")?.read_to_string(&mut buf)?;
    let value = buf.trim().parse::<f32>().unwrap();

    let result = format!(
        "  {}  {}°  {}",
        config.cpu_temperature.icon,
        value / 1000.0,
        config.seperator
    );
    Ok(result)
}
