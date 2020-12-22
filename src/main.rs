use std::process::Command;
use std::thread;
use std::time::Duration;

mod date;
mod disk;
mod load_config;
mod mem;

fn main() {
    let settings = load_config::load().unwrap();

    loop {
        let args = format!(
            "{}{}{}",
            disk::disk_free(&settings),
            mem::mem(&settings).unwrap(),
            date::fmt_date(&settings)
        );

        Command::new("xsetroot")
            .arg("-name")
            .arg(args)
            .output()
            .unwrap();

        thread::sleep(Duration::new(1, 0));
    }
}
