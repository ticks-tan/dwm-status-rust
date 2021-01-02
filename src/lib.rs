use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use yaml_rust::{yaml, YamlLoader};

#[derive(Debug)]
pub enum ThreadsData {
    Sound(String),
    Disk(String),
    Memory(String),
    Time(String),
}

#[derive(Clone)]
pub struct Config {
    pub seperator: String,
    pub time: Time,
    pub memory: Memory,
    pub disk: Disk,
    pub volume: Volume,
}

#[derive(Clone)]
pub struct Time {
    pub format: String,
    pub icon: String,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Memory {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Disk {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}
#[derive(Clone)]
pub struct Volume {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

pub fn load_config() -> Result<Config, Error> {
    let yml_source = env::var("HOME").unwrap() + "/.config/rsblocks/rsblocks.yml";
    let mut data = String::new();
    let mut file = match File::open(&yml_source) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{} file not found, loading defaults!", &yml_source);
            return Ok(load_defaults());
        }
    };
    file.read_to_string(&mut data)?;

    let yml_content = &YamlLoader::load_from_str(&data).unwrap()[0];
    let config = parse_config(yml_content);
    Ok(config)
}

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
        },
    }
}

fn parse_config(doc: &yaml::Yaml) -> Config {
    // parsing icons and set default if not exist in the config file
    let seperator = get_or_set_string(doc, "general", "seperator", "|");
    let time_icon = get_or_set_string(doc, "time", "icon", "");
    let time_format = get_or_set_string(doc, "time", "format", "%T");
    let mem_icon = get_or_set_string(doc, "memory", "icon", "");
    let disk_icon = get_or_set_string(doc, "disk", "icon", "");
    let volume_icon = get_or_set_string(doc, "volume", "icon", "");

    // parsing enabled state (everything false by default)
    let disk_enabled = get_or_set_bool(doc, "disk", "enable");
    let memory_enabled = get_or_set_bool(doc, "memory", "enable");
    let volume_enabled = get_or_set_bool(doc, "volume", "enable");

    // parsing update_delay state (should be all seconds in f64 type)
    let time_delay = get_or_set_f32(doc, "time", "delay", 1.0);
    let disk_delay = get_or_set_f32(doc, "disk", "delay", 60.0);
    let memory_delay = get_or_set_f32(doc, "memory", "delay", 2.0);
    let volume_delay = get_or_set_f32(doc, "volume", "delay", 0.17);

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
        },
    }
}

// getting a f32 value from rsblocks.yml file or set default (last argument)
fn get_or_set_f32(doc: &yaml::Yaml, parent: &str, child: &str, default: f64) -> f64 {
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

/*######################## LOADING CONFIGS ENDS HERE  ############################

*/

/* Running the program:
TODO: this is sucks, repeated code in threads below, fix me you fucking asshole
 */

pub fn run(config: Config) {
    let (tx, rx) = mpsc::channel();

    // volume thread
    if config.volume.enabled {
        let volume_tx = tx.clone();
        let configcp = config.clone();
        let mut vol_data = ThreadsData::Sound(get_volume(&configcp));
        thread::spawn(move || loop {
            let _ = volume_tx.send(vol_data);
            vol_data = ThreadsData::Sound(get_volume(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.volume.delay))
        });
    }

    // Disk thread
    if config.disk.enabled {
        let disk_tx = tx.clone();
        let configcp = config.clone();
        let mut disk_data = ThreadsData::Disk(get_disk(&configcp));
        thread::spawn(move || loop {
            disk_tx.send(disk_data).unwrap();
            disk_data = ThreadsData::Disk(get_disk(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.disk.delay))
        });
    }

    // Memory thread
    if config.memory.enabled {
        let memory_tx = tx.clone();
        let configcp = config.clone();
        let memory_data = get_memory(&configcp).unwrap();
        let mut memory_data = ThreadsData::Memory(memory_data);
        thread::spawn(move || loop {
            memory_tx.send(memory_data).unwrap();
            memory_data = ThreadsData::Memory(get_memory(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.memory.delay))
        });
    }

    // Time thread
    {
        let time_tx = tx;
        let configcp = config.clone();
        let mut time_data = ThreadsData::Time(get_time(&configcp));
        thread::spawn(move || loop {
            time_tx.send(time_data).unwrap();
            time_data = ThreadsData::Time(get_time(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.time.delay))
        });
    }

    //Main
    {
        // NOTE: order matters to the final format

        let mut bar: Vec<String> = vec!["".to_string(); 4];
        //iterating the values recieved from the threads
        for data in rx {
            match data {
                ThreadsData::Sound(x) => bar[0] = x,
                ThreadsData::Disk(x) => bar[1] = x,
                ThreadsData::Memory(x) => bar[2] = x,
                ThreadsData::Time(x) => bar[3] = x,
            }

            // match ends here
            update(&bar);
        }
    }
}

pub fn update(bar: &Vec<String>) {
    // TODO: FIX ME, this solution sucks
    let mut x = String::new();
    for i in bar.iter() {
        x.push_str(i.as_str());
    }
    Command::new("xsetroot")
        .arg("-name")
        .arg(x)
        .output()
        .unwrap();
}

/*############################ RUNNING THE PROGRAM ENDS HERE ###########################################*/

/*




############################# HELPER FUNCTIONS BELOW ###################################
*/
pub fn get_time(config: &Config) -> String {
    let now = Local::now();

    format!(
        "  {}  {}  {}",
        config.time.icon,
        now.format(&config.time.format),
        config.seperator
    )
}

pub fn get_disk(config: &Config) -> String {
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
//       find another way

pub fn get_volume(config: &Config) -> String {
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

pub fn get_memory(config: &Config) -> Result<String, std::io::Error> {
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
// helper function for the get_memory function
fn assign_val(line: &str, assignable: &mut u32) {
    let parsed: u32 = line.split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    *assignable = parsed;
}
