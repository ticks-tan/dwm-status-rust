mod battery;
mod cputemp;
mod disk;
mod memory;
mod mpd;
mod time;
mod uptime;
mod volume;
mod weather;
mod spotify;
mod loadavg;

use self::mpd::Mpd;
use battery::Battery;
use cputemp::CpuTemp;
use disk::Disk;
use memory::Memory;
use time::Time;
use uptime::Uptime;
use volume::Volume;
use weather::Weather;
use spotify::Spotify;
use loadavg::LoadAvg;

use std::default::Default;
use std::fs::File;
use std::io::Read;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Config = {
        let yml_source = std::env::var("HOME").unwrap() + "/.config/rsblocks/rsblocks.yml";
        let mut data = String::new();

        match File::open(&yml_source) {
            Ok(mut file) => {
                file.read_to_string(&mut data).expect("Failed to read config to string");
                serde_yaml::from_str(&data).expect("Failed to parse config")
            }
            Err(_) => Config::default(),
        }
    };
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_seperator")]
    pub seperator: String,

    #[serde(default)]
    pub time: Time,

    #[serde(default)]
    pub memory: Memory,

    #[serde(default)]
    pub disk: Disk,

    #[serde(default)]
    pub volume: Volume,

    #[serde(default)]
    pub weather: Weather,

    #[serde(default)]
    pub battery: Battery,

    #[serde(default)]
    pub cpu_temperature: CpuTemp,

    #[serde(default)]
    pub uptime: Uptime,

    #[serde(default)]
    pub mpd: Mpd,

    #[serde(default)]
    pub spotify: Spotify,

    #[serde(default)]
    pub loadavg: LoadAvg
}

impl Default for Config {
    fn default() -> Self {
        Self {
            seperator: "|".to_string(),
            time: Default::default(),
            memory: Default::default(),
            disk: Default::default(),
            volume: Default::default(),
            weather: Default::default(),
            battery: Default::default(),
            cpu_temperature: Default::default(),
            uptime: Default::default(),
            mpd: Default::default(),
            spotify: Default::default(),
            loadavg: Default::default()
        }
    }
}

fn default_seperator() -> String {
    "|".to_string()
}
