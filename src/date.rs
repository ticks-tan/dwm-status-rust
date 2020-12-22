use crate::load_config::Settings;
use chrono::prelude::*;

pub fn fmt_date(setting: &Settings) -> String {
    let now = Local::now();

    format!(
        "  {}  {}  {}",
        setting.time.icon,
        now.format(&setting.time.format),
        setting.seperator
    )
}
