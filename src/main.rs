use std::process::Command;
use std::thread;
use std::time::Duration;

mod date;
mod disk;
mod mem;

fn main() {
    loop {
        let args = format!(
            "{}{}{}",
            disk::disk_free(),
            mem::mem().unwrap(),
            date::fmt_date("%d %b, %I:%M:%S %p")
        );

        Command::new("xsetroot")
            .arg("-name")
            .arg(args)
            .output()
            .unwrap();

        thread::sleep(Duration::new(1, 0));
    }
}
