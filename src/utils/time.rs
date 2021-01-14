use crate::types::Config;
use chrono::prelude::*;

pub fn get_time(config: &Config) -> String {
    let now = Local::now();

    format!(
        "  {}  {}  {}",
        config.time.icon,
        now.format(&config.time.format),
        config.seperator
    )
}
