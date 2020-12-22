use std::env;
use std::{fs::File, io::Error, io::Read};
use yaml_rust::{yaml, YamlLoader};

pub struct Time {
    pub format: String,
    pub icon: String,
}

pub struct Memory {
    pub icon: String,
}

pub struct Disk {
    pub icon: String,
}

pub struct Settings {
    pub seperator: String,
    pub time: Time,
    pub memory: Memory,
    pub disk: Disk,
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

    let yml_doc = &YamlLoader::load_from_str(&data).unwrap()[0];
    let settings = gen_settings(yml_doc);
    Ok(settings)
}

fn gen_settings(doc: &yaml::Yaml) -> Settings {
    let seperator: String;
    let time_format: String;
    let time_icon: String;
    let mem_icon: String;
    let disk_icon: String;

    if doc["general"]["seperator"].is_badvalue() {
        seperator = String::from("|");
    } else {
        seperator = String::from(doc["general"]["seperator"].as_str().unwrap());
    }
    if doc["time"]["icon"].is_badvalue() {
        time_icon = String::from("")
    } else {
        time_icon = String::from(doc["time"]["icon"].as_str().unwrap());
    }
    if doc["time"]["format"].is_badvalue() {
        time_format = String::from("%T")
    } else {
        time_format = String::from(doc["time"]["format"].as_str().unwrap())
    }
    if doc["memory"]["icon"].is_badvalue() {
        mem_icon = String::from("")
    } else {
        mem_icon = String::from(doc["memory"]["icon"].as_str().unwrap());
    }
    if doc["disk"]["icon"].is_badvalue() {
        disk_icon = String::from("")
    } else {
        disk_icon = String::from(doc["disk"]["icon"].as_str().unwrap());
    }

    Settings {
        seperator,
        time: Time {
            format: time_format,
            icon: time_icon,
        },
        memory: Memory { icon: mem_icon },
        disk: Disk { icon: disk_icon },
    }
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
        },
        disk: Disk {
            icon: String::from(""),
        },
    }
}
