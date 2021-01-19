use crate::types::*;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use yaml_rust::{yaml, YamlLoader};
/*
this function is responsible to check if the rsblocks.yml file
exists to call parse_config to read it OTHERWISE
it will call load_defaults to load a hardcoded default configuration

it will always return a valid configuration inside a Result.
*/
pub fn load_config() -> Result<Config, Error> {
    let yml_source = std::env::var("HOME").unwrap() + "/.config/rsblocks/rsblocks.yml";
    let mut data = String::new();
    let mut file = match File::open(&yml_source) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{} file not found, loading defaults!", &yml_source);
            return Ok(load_defaults());
        }
    };
    file.read_to_string(&mut data)?;

    // checking if rsblocks.yml is empty
    let yml_content = match YamlLoader::load_from_str(&data) {
        Ok(content) => {
            if content.len() > 0 {
                content[0].clone()
            } else {
                eprintln!("configuration file looks empty, loading defaults!");
                return Ok(load_defaults());
            }
        }
        _ => return Ok(load_defaults()),
    };

    let config = parse_config(&yml_content);
    Ok(config)
}

/*
this is simply returns a hardcoded configuration as default
*/
fn load_defaults() -> Config {
    Config {
        seperator: String::from("|"),
        time: Time {
            format: String::from("%T"),
            icon: String::from(""),
            delay: 1.0,
        },
        memory: Memory {
            icon: String::from(""),
            enabled: true,
            delay: 2.0,
        },
        disk: Disk {
            icon: String::from(""),
            enabled: false,
            delay: 60.0,
        },
        volume: Volume {
            icon: String::from(""),
            enabled: false,
            delay: 0.17,
            card: String::from("ALSA"),
        },
        weather: Weather {
            city: String::from(""),
            format: String::from("+%t"),
            icon: String::from(""),
            enabled: false,
            delay: 7200.0, //7200 seconds = 2 hours
        },
        battery: Battery {
            source: String::from("BAT0"),
            icon: String::from(""),
            enabled: false,
            delay: 120.0,
        },
        cpu_temperature: CpuTemp {
            icon: String::from(""),
            enabled: false,
            delay: 120.0,
        },
        uptime: Uptime {
            icon: String::from(""),
            enabled: false,
            delay: 60.0,
        },
        mpd: Mpd {
            icon: String::from(""),
            host: String::from("127.0.0.1"),
            port: String::from("6600"),
            enabled: false,
            delay: 15.0,
        },
        spotify: Spotify {
            icon: String::from(""),
            enabled: false,
            delay: 15.0,
        },
        loadavg: LoadAvg {
            icon: String::from(""),
            enabled: false,
            delay: 60.0,
        },
    }
}

/*
it will read and parse the rsblocks.yml file content and return a valid configuration
IF some content is missing in the rsblocks.yml file, it will set
a default values to that.

NOTE: (get_or_set) functions job below getting the values from the configuration doc IF
      a value is not exist in the config it will SET a value givin in the last argument.
*/
fn parse_config(doc: &yaml::Yaml) -> Config {
    let seperator = get_or_set_string(doc, "general", "seperator", "|");

    // time values
    let time_icon = get_or_set_string(doc, "time", "icon", "");
    let time_format = get_or_set_string(doc, "time", "format", "%T");
    let time_delay = get_or_set_f64(doc, "time", "delay", 1.0);

    // memory values
    let mem_icon = get_or_set_string(doc, "memory", "icon", "");
    let memory_enabled = get_or_set_bool(doc, "memory", "enable");
    let memory_delay = get_or_set_f64(doc, "memory", "delay", 2.0);

    //disk values
    let disk_icon = get_or_set_string(doc, "disk", "icon", "");
    let disk_enabled = get_or_set_bool(doc, "disk", "enable");
    let disk_delay = get_or_set_f64(doc, "disk", "delay", 120.0);

    // volume values
    let volume_icon = get_or_set_string(doc, "volume", "icon", "");
    let volume_enabled = get_or_set_bool(doc, "volume", "enable");
    let volume_delay = get_or_set_f64(doc, "volume", "delay", 0.17);
    let volume_card = get_or_set_string(doc, "volume", "card", "ALSA");

    // weather values
    let weather_icon = get_or_set_string(doc, "weather", "icon", "");
    let weather_format = get_or_set_string(doc, "weather", "format", "%l:+%t");
    let weather_city = get_or_set_string(doc, "weather", "city", "");
    let weather_enabled = get_or_set_bool(doc, "weather", "enable");
    let weather_delay = get_or_set_f64(doc, "weather", "delay", 7200.0);

    // battery values
    let battery_icon = get_or_set_string(doc, "battery", "icon", "");
    let battery_enabled = get_or_set_bool(doc, "battery", "enable");
    let battery_source = get_or_set_string(doc, "battery", "source", "BAT0");
    let battery_delay = get_or_set_f64(doc, "battery", "delay", 120.0);

    // cpu values
    let cpu_temperature_icon = get_or_set_string(doc, "cpu_temperature", "icon", "");
    let cpu_temperature_enabled = get_or_set_bool(doc, "cpu_temperature", "enable");
    let cpu_temperature_delay = get_or_set_f64(doc, "cpu_temperature", "delay", 120.0);

    // uptime values
    let uptime_icon = get_or_set_string(doc, "uptime", "icon", "");
    let uptime_enabled = get_or_set_bool(doc, "uptime", "enable");
    let uptime_delay = get_or_set_f64(doc, "uptime", "delay", 60.0);

    // mpd values
    let mpd_icon = get_or_set_string(doc, "mpd", "icon", "");
    let mpd_host = get_or_set_string(doc, "mpd", "host", "127.0.0.1");
    let mpd_port = get_or_set_string(doc, "mpd", "port", "6600");
    let mpd_enabled = get_or_set_bool(doc, "mpd", "enable");
    let mpd_delay = get_or_set_f64(doc, "mpd", "delay", 15.0);

    //spotify values
    let spotify_icon = get_or_set_string(doc, "spotify", "icon", "");
    let spotify_enabled = get_or_set_bool(doc, "spotify", "enable");
    let spotify_delay = get_or_set_f64(doc, "spotify", "delay", 10.0);

    //Load Avrage values
    let loadavg_icon = get_or_set_string(doc, "loadavg", "icon", "");
    let loadavg_enabled = get_or_set_bool(doc, "loadavg", "enable");
    let loadavg_delay = get_or_set_f64(doc, "loadavg", "delay", 60.0);

    Config {
        seperator,
        time: Time {
            format: time_format,
            icon: time_icon,
            delay: time_delay,
        },
        memory: Memory {
            icon: mem_icon,
            enabled: memory_enabled,
            delay: memory_delay,
        },
        disk: Disk {
            icon: disk_icon,
            enabled: disk_enabled,
            delay: disk_delay,
        },
        volume: Volume {
            icon: volume_icon,
            enabled: volume_enabled,
            delay: volume_delay,
            card: volume_card,
        },
        weather: Weather {
            city: weather_city,
            format: weather_format,
            icon: weather_icon,
            enabled: weather_enabled,
            delay: weather_delay,
        },
        battery: Battery {
            source: battery_source,
            icon: battery_icon,
            enabled: battery_enabled,
            delay: battery_delay,
        },
        cpu_temperature: CpuTemp {
            icon: cpu_temperature_icon,
            enabled: cpu_temperature_enabled,
            delay: cpu_temperature_delay,
        },
        uptime: Uptime {
            icon: uptime_icon,
            enabled: uptime_enabled,
            delay: uptime_delay,
        },
        mpd: Mpd {
            icon: mpd_icon,
            host: mpd_host,
            port: mpd_port,
            enabled: mpd_enabled,
            delay: mpd_delay,
        },
        spotify: Spotify {
            icon: spotify_icon,
            enabled: spotify_enabled,
            delay: spotify_delay,
        },
        loadavg: LoadAvg {
            icon: loadavg_icon,
            enabled: loadavg_enabled,
            delay: loadavg_delay,
        },
    }
}

// getting a f64 value from rsblocks.yml file or set default (last argument)
fn get_or_set_f64(doc: &yaml::Yaml, parent: &str, child: &str, default: f64) -> f64 {
    let val: f64 = if doc[parent][child].is_badvalue() {
        default
    } else {
        doc[parent][child].as_f64().unwrap()
    };
    val
}

// getting a boolean value from rsblocks.yml file or set it false if it does not exist
fn get_or_set_bool(doc: &yaml::Yaml, parent: &str, child: &str) -> bool {
    let val: bool;
    if doc[parent][child].is_badvalue() {
        val = false;
    } else {
        val = doc[parent][child].as_bool().unwrap()
    }
    val
}

// getting a String value from the rsblocks.yml file or set the default(last argument)
fn get_or_set_string(doc: &yaml::Yaml, parent: &str, child: &str, default_val: &str) -> String {
    let val: String;
    if doc[parent][child].is_badvalue() {
        val = String::from(default_val)
    } else {
        val = String::from(doc[parent][child].as_str().unwrap());
    }

    val
}
