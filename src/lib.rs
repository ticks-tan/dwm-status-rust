use alsa::mixer::{Mixer, SelemChannelId, SelemId};
use breadx::{display::*, window::Window};
use chrono::prelude::*;
use std::fs::File;
use std::io::Error;
use std::io::Read;
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
    Weather(String),
    Battery(String),
    CpuTemp(String),
    Uptime(String),
}

#[derive(Clone)]
pub struct Config {
    pub seperator: String,
    pub time: Time,
    pub memory: Memory,
    pub disk: Disk,
    pub volume: Volume,
    pub weather: Weather,
    pub battery: Battery,
    pub cpu_temperature: CpuTemp,
    pub uptime: Uptime,
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
    pub card: String,
}

#[derive(Clone)]
pub struct Weather {
    pub city: String,
    pub format: String,
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Battery {
    pub source: String,
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct CpuTemp {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Uptime {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}
/*                            TODOS

    TODO 1: Error handling required if rsblocks.yml file is empty.

    TODO 2: This lib file growing and soon it will be annoying to move
            arround, need fix soon.

    TODO 3: Need a better comments in code, or no one will understand what happens.

    TODO 4: Need a documentation.

    TODO 5: Fix repeated code for threads in `run` function.

*/

/*this function is responsible to check if the rsblocks.yml file
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
    let yml_content = &YamlLoader::load_from_str(&data).unwrap()[0];
    let config = parse_config(yml_content);
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
    }
}

/*
it will read and parse the rsblocks.yml file content and return a valid configuration
IF some content is missing in the rsblocks.yml file, it will set
a default values to that
*/
fn parse_config(doc: &yaml::Yaml) -> Config {
    // parsing icons and set default if not exist in the config file
    let seperator = get_or_set_string(doc, "general", "seperator", "|");
    let time_icon = get_or_set_string(doc, "time", "icon", "");
    let mem_icon = get_or_set_string(doc, "memory", "icon", "");
    let disk_icon = get_or_set_string(doc, "disk", "icon", "");
    let volume_icon = get_or_set_string(doc, "volume", "icon", "");
    let weather_icon = get_or_set_string(doc, "weather", "icon", "");
    let battery_icon = get_or_set_string(doc, "battery", "icon", "");
    let cpu_temperature_icon = get_or_set_string(doc, "cpu_temperature", "icon", "");
    let uptime_icon = get_or_set_string(doc, "uptime", "icon", "");

    //parsing formats and city weather
    let time_format = get_or_set_string(doc, "time", "format", "%T");
    let weather_format = get_or_set_string(doc, "weather", "format", "%l:+%t");
    let weather_city = get_or_set_string(doc, "weather", "city", "");

    // parsing enabled state (everything false by default)
    let disk_enabled = get_or_set_bool(doc, "disk", "enable");
    let memory_enabled = get_or_set_bool(doc, "memory", "enable");
    let volume_enabled = get_or_set_bool(doc, "volume", "enable");
    let weather_enabled = get_or_set_bool(doc, "weather", "enable");
    let battery_enabled = get_or_set_bool(doc, "battery", "enable");
    let cpu_temperature_enabled = get_or_set_bool(doc, "cpu_temperature", "enable");
    let uptime_enabled = get_or_set_bool(doc, "uptime", "enable");

    // parsing update_delay state (should be all seconds in f64 type)
    let time_delay = get_or_set_f64(doc, "time", "delay", 1.0);
    let disk_delay = get_or_set_f64(doc, "disk", "delay", 120.0);
    let memory_delay = get_or_set_f64(doc, "memory", "delay", 2.0);
    let volume_delay = get_or_set_f64(doc, "volume", "delay", 0.17);
    let weather_delay = get_or_set_f64(doc, "weather", "delay", 7200.0);
    let battery_delay = get_or_set_f64(doc, "battery", "delay", 120.0);
    let cpu_temperature_delay = get_or_set_f64(doc, "cpu_temperature", "delay", 120.0);
    let uptime_delay = get_or_set_f64(doc, "uptime", "delay", 60.0);

    // parsing card for volume, ALSA or PULSE
    let volume_card = get_or_set_string(doc, "volume", "card", "ALSA");

    // parsing battery source name
    let battery_source = get_or_set_string(doc, "battery", "source", "BAT0");

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
    }
}

// getting a f32 value from rsblocks.yml file or set default (last argument)
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

/*######################## LOADING CONFIGS ENDS HERE  ############################

*/

// Running the program:

pub struct Blocks {
    disp: Display<name::NameConnection>,
    root: Window,
}

impl Blocks {
    pub fn new() -> Self {
        let disp = Display::create(None, None).expect("Failed to create x11 connection");
        let root = disp.default_screen().root;
        Self { disp, root }
    }
}

impl Default for Blocks {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run(config: Config, mut blocks: Blocks) {
    let (tx, rx) = mpsc::channel();
    // volume thread
    if config.volume.enabled {
        let volume_tx = tx.clone();
        let configcp = config.clone();
        let mut vol_data = ThreadsData::Sound(get_volume(&configcp));
        thread::spawn(move || loop {
            volume_tx.send(vol_data).unwrap();
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

    // Weather thread
    if config.weather.enabled {
        let weather_tx = tx.clone();
        let configcp = config.clone();
        let weather_data = get_weather(&configcp);
        let mut weather_data = ThreadsData::Weather(weather_data);
        thread::spawn(move || loop {
            weather_tx.send(weather_data).unwrap();
            weather_data = ThreadsData::Weather(get_weather(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.weather.delay))
        });
    }

    // Battery thread
    if config.battery.enabled {
        let battery_tx = tx.clone();
        let configcp = config.clone();
        let battery_data = get_battery(&configcp).unwrap();
        let mut battery_data = ThreadsData::Battery(battery_data);
        thread::spawn(move || loop {
            battery_tx.send(battery_data).unwrap();
            battery_data = ThreadsData::Battery(get_battery(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.battery.delay))
        });
    }

    // Cpu temperature thread
    if config.cpu_temperature.enabled {
        let cpu_temp_tx = tx.clone();
        let configcp = config.clone();
        let cpu_temp_data = get_cpu_temp(&configcp).unwrap();
        let mut cpu_temp_data = ThreadsData::CpuTemp(cpu_temp_data);
        thread::spawn(move || loop {
            cpu_temp_tx.send(cpu_temp_data).unwrap();
            cpu_temp_data = ThreadsData::CpuTemp(get_cpu_temp(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.cpu_temperature.delay))
        });
    }

    // Uptime thread
    if config.uptime.enabled {
        let uptime_tx = tx.clone();
        let configcp = config.clone();
        let uptime_data = get_uptime(&configcp).unwrap();
        let mut uptime_data = ThreadsData::Uptime(uptime_data);
        thread::spawn(move || loop {
            uptime_tx.send(uptime_data).unwrap();
            uptime_data = ThreadsData::Uptime(get_uptime(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.uptime.delay))
        });
    }
    // Time thread
    {
        let time_tx = tx;
        let configcp = config;
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
        let mut bar: Vec<String> = vec!["".to_string(); 8];
        //iterating the values recieved from the threads
        for data in rx {
            match data {
                ThreadsData::Sound(x) => bar[0] = x,
                ThreadsData::Weather(x) => bar[1] = x,
                ThreadsData::Disk(x) => bar[2] = x,
                ThreadsData::Memory(x) => bar[3] = x,
                ThreadsData::CpuTemp(x) => bar[4] = x,
                ThreadsData::Battery(x) => bar[5] = x,
                ThreadsData::Uptime(x) => bar[6] = x,
                ThreadsData::Time(x) => bar[7] = x,
            }

            // match ends here
            update(&bar, &mut blocks);
        }
    }
}

pub fn update(bar: &[String], blocks: &mut Blocks) {
    // TODO: FIX ME, this solution sucks
    let mut x = String::new();
    for i in bar.iter() {
        x.push_str(i.as_str());
    }

    blocks
        .root
        .set_title(&mut blocks.disp, &x)
        .expect("Failed to set title of root");
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

/*
CREDIT: thanks for wttr.in to use their API
will make a GET request from wttr.in
*/
fn get_weather(config: &Config) -> String {
    let format = if config.weather.format.is_empty() {
        String::from("%l:+%t")
    } else {
        config.weather.format.clone()
    };

    let url = format!("http://wttr.in/{}?format=\"{}", config.weather.city, format);
    let err_string = String::from("Error");
    let res = match minreq::get(url).send() {
        Ok(resp) => match resp.as_str() {
            Ok(res_str) => res_str.trim_matches('"').to_string(),
            Err(_) => err_string,
        },
        Err(_) => err_string,
    };

    format!("  {}  {}  {}", config.weather.icon, res, config.seperator)
}

// getting disk usage
pub fn get_disk(config: &Config) -> String {
    const GB: u64 = (1024 * 1024) * 1024;
    let statvfs = nix::sys::statvfs::statvfs("/").unwrap();
    let mut disk_used = String::new();

    let total = (statvfs.blocks() * statvfs.fragment_size()) / GB;
    let available = (statvfs.blocks_free() * statvfs.fragment_size()) / GB;
    let used = total - available;

    disk_used.push_str(&format!("{}G", used));
    format!(
        "  {}  {}  {}",
        config.disk.icon, disk_used, config.seperator
    )
}

pub fn get_volume(config: &Config) -> String {
    let card = if config.volume.card == "PULSE" {
        "pulse"
    } else {
        "default"
    };

    let mixer = Mixer::new(card, false).expect("Failed to open mixer");
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&selem_id).expect("Couldn't find selem");
    let selem_chan_id = SelemChannelId::FrontLeft;

    let (min, max) = selem.get_playback_volume_range();
    let mut raw_volume = selem
        .get_playback_volume(selem_chan_id)
        .expect("Failed to get raw_volume");

    let range = max - min;
    let vol = if range == 0 {
        0
    } else {
        raw_volume -= min;
        ((raw_volume as f64 / range as f64) * 100.) as u64
    };

    format!("  {}  {}%  {}", config.volume.icon, vol, config.seperator)
}

/*
mem_used = (mem_total + shmem - mem_free - mem_buffers - mem_cached - mem_srecl
thanks for htop's developer on stackoverflow for providing this algorithm to
calculate used memory.
*/
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

/*
this helper function will split the line(first argument) by the character(:)
and then parse the right splited item as u32
then assign that to the "assignable"(2nd argument).
*/
fn assign_val(line: &str, assignable: &mut u32) {
    let parsed: u32 = line.split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    *assignable = parsed;
}

// getting battery percentage
pub fn get_battery(config: &Config) -> Result<String, Error> {
    let battery_full_cap_file = format!(
        "/sys/class/power_supply/{}/charge_full_design",
        config.battery.source
    );
    let battery_charge_now_file = format!(
        "/sys/class/power_supply/{}/charge_now",
        config.battery.source
    );

    let mut buf = String::new();

    // FIXME: ugly error handling AGAIN fixing later, im lazy
    match File::open(&battery_full_cap_file) {
        Ok(mut file) => file.read_to_string(&mut buf)?,
        Err(_) => return Ok(String::from("check your battery source name")),
    };
    let full_design = buf.trim().parse::<u32>().unwrap();
    buf.clear();

    // NOTE: no need to error check if passed the above match
    File::open(&battery_charge_now_file)?.read_to_string(&mut buf)?;

    let charge_now = buf.trim().parse::<u32>().unwrap();

    let battery_percentage = (charge_now as f32 / full_design as f32) * 100.0;
    let result = format!(
        "  {}  {:.0}%  {}",
        config.battery.icon, battery_percentage, config.seperator
    );
    Ok(result)
}

// getting cpu temperature
pub fn get_cpu_temp(config: &Config) -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/sys/class/thermal/thermal_zone0/temp")?.read_to_string(&mut buf)?;
    let value = buf.trim().parse::<f32>().unwrap();

    let result = format!(
        "  {}  {}°  {}",
        config.cpu_temperature.icon,
        value / 1000.0,
        config.seperator
    );
    Ok(result)
}

pub fn get_uptime(config: &Config) -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut buf)?;

    let buf: f32 = buf.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();

    let hour = buf.round() as u32 / 3600;
    let rem = buf as u32 - hour * 3600;
    let minutes = rem / 60;

    let uptime = if hour > 0 {
        format!("{}:{}", hour, minutes)
    } else {
        format!("{} min", minutes)
    };
    let result = format!("  {}  {}  {}", config.uptime.icon, uptime, config.seperator);
    Ok(result)
}
