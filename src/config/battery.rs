use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Battery {
    pub source: String,
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

impl Default for Battery {
    fn default() -> Self {
        Battery {
            source: String::from("BAT0"),
            icon: String::from(""),
            enabled: false,
            delay: 120.0,
        }
    }
}
