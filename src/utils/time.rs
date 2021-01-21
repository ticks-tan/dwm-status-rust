use crate::config::CONFIG;
use chrono::prelude::*;

pub fn get_time() -> String {
    let now = Local::now();

    format!(
        "  {}  {}  {}",
        CONFIG.time.icon,
        now.format(&CONFIG.time.format),
        CONFIG.seperator
    )
}
