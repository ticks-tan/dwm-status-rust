use std::env;
use std::{fs::File, io::Error, io::Read};
use yaml_rust::{yaml, YamlLoader};

pub struct Time {
    pub format: String,
    pub icon: String,
}

pub struct Memory {
    pub icon: String,
    pub enabled: bool,
}

pub struct Disk {
    pub icon: String,
    pub enabled: bool,
}

pub struct Volume {
    pub icon: String,
    pub enabled: bool,
}

pub struct Settings {
    pub seperator: String,
    pub time: Time,
    pub memory: Memory,
    pub disk: Disk,
    pub volume: Volume,
}

pub fn load() -> Result<Settings, Error> {
    let yml_source = env::var("HOME").unwrap() + "/.config/rsblocks/rsblocks.yml";
    let mut data = String::new();
    let mut file = match File::open(yml_source) {
        Ok(file) => file,
        Err(_) => {
            println!("~/.config/rsblocks/rsblocks.yml file not found, loading defaults!");
            return Ok(load_defaults());
        }
    };
    file.read_to_string(&mut data)?;

    let yml_content = &YamlLoader::load_from_str(&data).unwrap()[0];
    let settings = gen_settings(yml_content);
    Ok(settings)
}

fn gen_settings(doc: &yaml::Yaml) -> Settings {
    // setting icons
    let seperator = get_or_set_string(doc, "general", "seperator", "|");
    let time_icon = get_or_set_string(doc, "time", "icon", "");
    let time_format = get_or_set_string(doc, "time", "format", "%T");
    let mem_icon = get_or_set_string(doc, "memory", "icon", "");
    let disk_icon = get_or_set_string(doc, "disk", "icon", "");
    let volume_icon = get_or_set_string(doc, "volume", "icon", "");

    // setting enable status, everything false by default
    let disk_enabled = get_or_set_bool(doc, "disk", "enable");
    let memory_enabled = get_or_set_bool(doc, "memory", "enable");
    let volume_enabled = get_or_set_bool(doc, "volume", "enable");

    Settings {
        seperator,
        time: Time {
            format: time_format,
            icon: time_icon,
        },
        memory: Memory {
            icon: mem_icon,
            enabled: memory_enabled,
        },
        disk: Disk {
            icon: disk_icon,
            enabled: disk_enabled,
        },
        volume: Volume {
            icon: volume_icon,
            enabled: volume_enabled,
        },
    }
}

// getting the bool value from rsblocks.yml file or set it false if it does not exist
fn get_or_set_bool(doc: &yaml::Yaml, parent: &str, child: &str) -> bool {
    let val: bool;
    if doc[parent][child].is_badvalue() {
        val = false;
    } else {
        val = doc[parent][child].as_bool().unwrap()
    }
    val
}

// getting the value from the rsblocks.yml file or set the default in the last parameter
fn get_or_set_string(doc: &yaml::Yaml, parent: &str, child: &str, default_val: &str) -> String {
    let val: String;
    if doc[parent][child].is_badvalue() {
        val = String::from(default_val)
    } else {
        val = String::from(doc[parent][child].as_str().unwrap());
    }

    val
}

fn load_defaults() -> Settings {
    Settings {
        seperator: String::from("|"),
        time: Time {
            format: String::from("%T"),
            icon: String::from(""),
        },
        memory: Memory {
            icon: String::from(""),
            enabled: false,
        },
        disk: Disk {
            icon: String::from(""),
            enabled: false,
        },
        volume: Volume {
            icon: String::from(""),
            enabled: false,
        },
    }
}
