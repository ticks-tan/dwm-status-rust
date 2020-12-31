use std::process::Command;
use std::thread;
use std::time::Duration;

mod date;
mod disk;
mod load_config;
mod mem;
mod sound;

// TODO: this is sucks, maybe thread, i want to spawn some threads

fn main() {
    let settings = load_config::load().unwrap();
    sound::get_sound(&settings);
    loop {
        let mut bar = String::from("");

        // the order of the IF's below matters to the final format

        if settings.volume.enabled {
            // volume return String
            bar.push_str(&sound::get_sound(&settings));
        }

        if settings.disk.enabled {
            // disk_free return String
            bar.push_str(&disk::disk_free(&settings));
        }

        if settings.memory.enabled {
            // mem return Result
            bar.push_str(&mem::mem(&settings).unwrap());
        }

        bar.push_str(&date::fmt_date(&settings));

        Command::new("xsetroot")
            .arg("-name")
            .arg(bar)
            .output()
            .unwrap();

        thread::sleep(Duration::from_millis(75));
    }
}
