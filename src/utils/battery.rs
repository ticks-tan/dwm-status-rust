use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::fs::read_to_string;

// getting battery percentage
pub fn get_battery() -> ThreadsData {
    let error = ThreadsData::Battery(String::from("check your battery source name"));

    let battery_full_cap_file = format!(
        "/sys/class/power_supply/{}/charge_full_design",
        CONFIG.battery.source
    );
    let battery_charge_now_file = format!(
        "/sys/class/power_supply/{}/charge_now",
        CONFIG.battery.source
    );

    let buf = match read_to_string(battery_full_cap_file) {
        Ok(file) => file,
        Err(_) => return error,
    };
    let full_design = buf.trim().parse::<u32>().unwrap();

    let buf = match read_to_string(&battery_charge_now_file) {
        Ok(data) => data,
        _ => return error,
    };

    let charge_now = buf.trim().parse::<u32>().unwrap();

    let battery_percentage = (charge_now as f32 / full_design as f32) * 100.0;
    let result = format!(
        "  {}  {:.0}%  {}",
        CONFIG.battery.icon, battery_percentage, CONFIG.seperator
    );
    ThreadsData::Battery(result)
}
