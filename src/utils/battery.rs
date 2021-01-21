use crate::config::CONFIG;
use std::fs::File;
use std::io::Error;
use std::io::Read;

// getting battery percentage
pub fn get_battery() -> Result<String, Error> {
    let battery_full_cap_file = format!(
        "/sys/class/power_supply/{}/charge_full_design",
        CONFIG.battery.source
    );
    let battery_charge_now_file = format!(
        "/sys/class/power_supply/{}/charge_now",
        CONFIG.battery.source
    );

    let mut buf = String::new();

    match File::open(&battery_full_cap_file) {
        Ok(mut file) => file.read_to_string(&mut buf)?,
        Err(_) => return Ok(String::from("check your battery source name")),
    };
    let full_design = buf.trim().parse::<u32>().unwrap();
    buf.clear();

    // No need to error check if passed the above match
    File::open(&battery_charge_now_file)?.read_to_string(&mut buf)?;

    let charge_now = buf.trim().parse::<u32>().unwrap();

    let battery_percentage = (charge_now as f32 / full_design as f32) * 100.0;
    let result = format!(
        "  {}  {:.0}%  {}",
        CONFIG.battery.icon, battery_percentage, CONFIG.seperator
    );
    Ok(result)
}
