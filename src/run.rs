use crate::types::*;
use crate::utils::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/* This is ugly, maybe i will try to impliment a threadpool  */

pub fn run(config: Config, mut blocks: Blocks) {
    let (tx, rx) = mpsc::channel();

    // loadavrage thread
    if config.loadavg.enabled {
        let loadavg_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let loadavg_data = ThreadsData::LoadAvg(load_average::get_load_avg(&configcp));
            loadavg_tx.send(loadavg_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.loadavg.delay))
        });
    }
    // spotify thread
    if config.spotify.enabled {
        let spotify_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let spotify_data = ThreadsData::Spotify(spotify::get_spotify(&configcp));
            spotify_tx.send(spotify_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.spotify.delay))
        });
    }

    // mpd thread
    if config.mpd.enabled {
        let mpd_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let mpd_data = ThreadsData::Mpd(mpd::get_mpd_current(&configcp));
            mpd_tx.send(mpd_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.mpd.delay))
        });
    }

    // volume thread
    if config.volume.enabled {
        let volume_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let vol_data = ThreadsData::Sound(volume::get_volume(&configcp));
            volume_tx.send(vol_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.volume.delay))
        });
    }

    // Disk thread
    if config.disk.enabled {
        let disk_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let disk_data = ThreadsData::Disk(disk::get_disk(&configcp));
            disk_tx.send(disk_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.disk.delay))
        });
    }

    // Memory thread
    if config.memory.enabled {
        let memory_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let memory_data = ThreadsData::Memory(memory::get_memory(&configcp).unwrap());
            memory_tx.send(memory_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.memory.delay))
        });
    }

    // Weather thread
    if config.weather.enabled {
        let weather_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let weather_data = ThreadsData::Weather(weather::get_weather(&configcp));
            weather_tx.send(weather_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.weather.delay))
        });
    }

    // Battery thread
    if config.battery.enabled {
        let battery_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let battery_data = ThreadsData::Battery(battery::get_battery(&configcp).unwrap());
            battery_tx.send(battery_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.battery.delay))
        });
    }

    // Cpu temperature thread
    if config.cpu_temperature.enabled {
        let cpu_temp_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let cpu_temp_data = ThreadsData::CpuTemp(cpu::get_cpu_temp(&configcp).unwrap());
            cpu_temp_tx.send(cpu_temp_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.cpu_temperature.delay))
        });
    }

    // Uptime thread
    if config.uptime.enabled {
        let uptime_tx = tx.clone();
        let configcp = config.clone();
        thread::spawn(move || loop {
            let uptime_data = ThreadsData::Uptime(uptime::get_uptime(&configcp).unwrap());
            uptime_tx.send(uptime_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.uptime.delay))
        });
    }

    // Time thread
    {
        let time_tx = tx;
        let configcp = config;
        thread::spawn(move || loop {
            let time_data = ThreadsData::Time(time::get_time(&configcp));
            time_tx.send(time_data).unwrap();
            thread::sleep(Duration::from_secs_f64(configcp.time.delay))
        });
    }

    //Main
    {
        // NOTE: order matters to the final format
        let mut bar: Vec<String> = vec![String::from(""); 11];
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
                ThreadsData::LoadAvg(x) => bar[7] = x,
                ThreadsData::Battery(x) => bar[8] = x,
                ThreadsData::Uptime(x) => bar[9] = x,
                ThreadsData::Time(x) => bar[10] = x,
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
