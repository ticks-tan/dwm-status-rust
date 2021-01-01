use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::process::Command;
use std::thread;
use std::time::Duration;
use yaml_rust::{yaml, YamlLoader};

pub struct Config {
    pub seperator: String,
    pub time: Time,
    pub memory: Memory,
    pub disk: Disk,
    pub volume: Volume,
}

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

pub fn load_config() -> Result<Config, Error> {
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
    let config = gen_default_config(yml_content);
    Ok(config)
}

fn gen_default_config(doc: &yaml::Yaml) -> Config {
    // setting icons
    let seperator = get_or_set_string(doc, "general", "seperator", "|");
    let time_icon = get_or_set_string(doc, "time", "icon", "");
    let time_format = get_or_set_string(doc, "time", "format", "%T");
    let mem_icon = get_or_set_string(doc, "memory", "icon", "");
    let disk_icon = get_or_set_string(doc, "disk", "icon", "");
    let volume_icon = get_or_set_string(doc, "volume", "icon", "");

    // everything false by default
    let disk_enabled = get_or_set_bool(doc, "disk", "enable");
    let memory_enabled = get_or_set_bool(doc, "memory", "enable");
    let volume_enabled = get_or_set_bool(doc, "volume", "enable");

    Config {
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

// getting a String value from the rsblocks.yml file or set the default in the last parameter
fn get_or_set_string(doc: &yaml::Yaml, parent: &str, child: &str, default_val: &str) -> String {
    let val: String;
    if doc[parent][child].is_badvalue() {
        val = String::from(default_val)
    } else {
        val = String::from(doc[parent][child].as_str().unwrap());
    }

    val
}

fn load_defaults() -> Config {
    Config {
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
/* LOADING CONFIGS ENDS HERE  */

/* Running the program:

TODO: this is sucks, i want to update each one in a diffrent delay, maybe i'll try to use threads
*/
pub fn run(config: Config) {
    loop {
        let mut bar = String::from("");

        // the order of the IF's below matters to the final format

        if config.volume.enabled {
            // volume return String
            bar.push_str(&get_sound(&config));
        }

        if config.disk.enabled {
            // disk_free return String
            bar.push_str(&disk_free(&config));
        }

        if config.memory.enabled {
            // mem return Result
            bar.push_str(&mem(&config).unwrap());
        }

        bar.push_str(&fmt_date(&config));

        Command::new("xsetroot")
            .arg("-name")
            .arg(bar)
            .output()
            .unwrap();

        thread::sleep(Duration::from_millis(75));
    }
}

// format bar date/time
pub fn fmt_date(config: &Config) -> String {
    let now = Local::now();

    format!(
        "  {}  {}  {}",
        config.time.icon,
        now.format(&config.time.format),
        config.seperator
    )
}

pub fn disk_free(config: &Config) -> String {
    let cmd = Command::new("sh")
        .arg("-c")
        .args(&["df -h"])
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&cmd.stdout);
    let mut disk_used = String::new();
    for line in output.lines() {
        if line.ends_with('/') {
            let splited = line.split_whitespace().collect::<Vec<&str>>();
            disk_used = splited[2].to_string();
            break;
        }
    }
    format!(
        "  {}  {}  {}",
        config.disk.icon, disk_used, config.seperator
    )
}

// TODO: what a horrible solution to get the sound, i dont like it
//       find another way you dumb fuck

pub fn get_sound(config: &Config) -> String {
    let cmd_content = Command::new("amixer")
        .args(&["-D", "pulse", "get", "Master"])
        .output()
        .expect("Make sure that you have alsa-utils installed on your system");

    let vol: String = String::from_utf8_lossy(&cmd_content.stdout)
        .lines()
        .last()
        .expect("failed to get sound volume")
        .split('[')
        .collect::<Vec<&str>>()[1]
        .split(']')
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_string();

    format!("  {}  {}  {}", config.volume.icon, vol, config.seperator)
}

pub fn mem(config: &Config) -> Result<String, std::io::Error> {
    let mut buf = String::new();

    File::open("/proc/meminfo")?.read_to_string(&mut buf)?;

    let mut mem_total: u32 = 0;
    let mut shmem: u32 = 0;
    let mut mem_free: u32 = 0;
    let mut mem_buffers: u32 = 0;
    let mut mem_cached: u32 = 0;
    let mut mem_srecl: u32 = 0;

    for line in buf.lines() {
        if mem_total > 0
            && shmem > 0
            && mem_free > 0
            && mem_buffers > 0
            && mem_cached > 0
            && mem_srecl > 0
        {
            break;
        }
        if line.starts_with("MemTotal") {
            assign_val(line, &mut mem_total);
        }
        if line.starts_with("SReclaimable") {
            assign_val(line, &mut mem_srecl)
        }
        if line.starts_with("Cached") {
            assign_val(line, &mut mem_cached)
        }

        if line.starts_with("Shmem") {
            assign_val(line, &mut shmem);
        }

        if line.starts_with("MemFree") {
            assign_val(line, &mut mem_free);
        }
        if line.starts_with("Buffers") {
            assign_val(line, &mut mem_buffers);
        }
    }

    let mem_used = (mem_total + shmem - mem_free - mem_buffers - mem_cached - mem_srecl) / 1024;
    let result: String;
    if mem_used > 1000 {
        result = format!(
            "  {}  {:.1}G  {}",
            config.memory.icon,
            mem_used as f32 / 1000.0,
            config.seperator
        );
    } else {
        result = format!(
            "  {}  {}M  {}",
            config.memory.icon, mem_used, config.seperator
        );
    }
    Ok(result)
}

fn assign_val(line: &str, assignable: &mut u32) {
    let parsed: u32 = line.split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    *assignable = parsed;
}
