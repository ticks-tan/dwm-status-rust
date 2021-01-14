use crate::types::*;
use crate::utils::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// FIXME: The most part i hate is this, looks really ugly, fix me you dumb fuck
pub fn run(config: Config, mut blocks: Blocks) {
    let (tx, rx) = mpsc::channel();

    // spotify thread
    if config.spotify.enabled {
        let spotify_tx = tx.clone();
        let configcp = config.clone();
        let mut spotify_data = ThreadsData::Spotify(spotify::get_spotify(&configcp));
        thread::spawn(move || loop {
            spotify_tx.send(spotify_data).unwrap();
            spotify_data = ThreadsData::Spotify(spotify::get_spotify(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.spotify.delay))
        });
    }

    // mpd thread
    if config.mpd.enabled {
        let mpd_tx = tx.clone();
        let configcp = config.clone();
        let mut mpd_data = ThreadsData::Mpd(mpd::get_mpd_current(&configcp));
        thread::spawn(move || loop {
            mpd_tx.send(mpd_data).unwrap();
            mpd_data = ThreadsData::Mpd(mpd::get_mpd_current(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.mpd.delay))
        });
    }

    // volume thread
    if config.volume.enabled {
        let volume_tx = tx.clone();
        let configcp = config.clone();
        let mut vol_data = ThreadsData::Sound(volume::get_volume(&configcp));
        thread::spawn(move || loop {
            volume_tx.send(vol_data).unwrap();
            vol_data = ThreadsData::Sound(volume::get_volume(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.volume.delay))
        });
    }

    // Disk thread
    if config.disk.enabled {
        let disk_tx = tx.clone();
        let configcp = config.clone();
        let mut disk_data = ThreadsData::Disk(disk::get_disk(&configcp));
        thread::spawn(move || loop {
            disk_tx.send(disk_data).unwrap();
            disk_data = ThreadsData::Disk(disk::get_disk(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.disk.delay))
        });
    }

    // Memory thread
    if config.memory.enabled {
        let memory_tx = tx.clone();
        let configcp = config.clone();
        let memory_data = memory::get_memory(&configcp).unwrap();
        let mut memory_data = ThreadsData::Memory(memory_data);
        thread::spawn(move || loop {
            memory_tx.send(memory_data).unwrap();
            memory_data = ThreadsData::Memory(memory::get_memory(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.memory.delay))
        });
    }

    // Weather thread
    if config.weather.enabled {
        let weather_tx = tx.clone();
        let configcp = config.clone();
        let weather_data = weather::get_weather(&configcp);
        let mut weather_data = ThreadsData::Weather(weather_data);
        thread::spawn(move || loop {
            weather_tx.send(weather_data).unwrap();
            weather_data = ThreadsData::Weather(weather::get_weather(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.weather.delay))
        });
    }

    // Battery thread
    if config.battery.enabled {
        let battery_tx = tx.clone();
        let configcp = config.clone();
        let battery_data = battery::get_battery(&configcp).unwrap();
        let mut battery_data = ThreadsData::Battery(battery_data);
        thread::spawn(move || loop {
            battery_tx.send(battery_data).unwrap();
            battery_data = ThreadsData::Battery(battery::get_battery(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.battery.delay))
        });
    }

    // Cpu temperature thread
    if config.cpu_temperature.enabled {
        let cpu_temp_tx = tx.clone();
        let configcp = config.clone();
        let cpu_temp_data = cpu::get_cpu_temp(&configcp).unwrap();
        let mut cpu_temp_data = ThreadsData::CpuTemp(cpu_temp_data);
        thread::spawn(move || loop {
            cpu_temp_tx.send(cpu_temp_data).unwrap();
            cpu_temp_data = ThreadsData::CpuTemp(cpu::get_cpu_temp(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.cpu_temperature.delay))
        });
    }

    // Uptime thread
    if config.uptime.enabled {
        let uptime_tx = tx.clone();
        let configcp = config.clone();
        let uptime_data = uptime::get_uptime(&configcp).unwrap();
        let mut uptime_data = ThreadsData::Uptime(uptime_data);
        thread::spawn(move || loop {
            uptime_tx.send(uptime_data).unwrap();
            uptime_data = ThreadsData::Uptime(uptime::get_uptime(&configcp).unwrap());
            thread::sleep(Duration::from_secs_f64(configcp.uptime.delay))
        });
    }

    // Time thread
    {
        let time_tx = tx;
        let configcp = config;
        let mut time_data = ThreadsData::Time(time::get_time(&configcp));
        thread::spawn(move || loop {
            time_tx.send(time_data).unwrap();
            time_data = ThreadsData::Time(time::get_time(&configcp));
            thread::sleep(Duration::from_secs_f64(configcp.time.delay))
        });
    }

    //Main
    {
        // NOTE: order matters to the final format
        let mut bar: Vec<String> = vec!["".to_string(); 10];
        //iterating the values recieved from the threads
        for data in rx {
            match data {
                ThreadsData::Spotify(x) => bar[0] = x,
                ThreadsData::Mpd(x) => bar[1] = x,
                ThreadsData::Sound(x) => bar[2] = x,
                ThreadsData::Weather(x) => bar[3] = x,
                ThreadsData::Disk(x) => bar[4] = x,
                ThreadsData::Memory(x) => bar[5] = x,
                ThreadsData::CpuTemp(x) => bar[6] = x,
                ThreadsData::Battery(x) => bar[7] = x,
                ThreadsData::Uptime(x) => bar[8] = x,
                ThreadsData::Time(x) => bar[9] = x,
            }

            // match ends here
            update(&bar, &mut blocks);
        }
    }
}

fn update(bar: &[String], blocks: &mut Blocks) {
    let mut x = String::new();
    for i in bar.iter() {
        x.push_str(i.as_str());
    }

    blocks
        .root
        .set_title(&mut blocks.disp, &x)
        .expect("Failed to set title of root");
}
