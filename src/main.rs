extern crate core;

mod config;
mod run;
mod types;
mod utils;
mod block_manager;
use block_manager::BlockManager;

use std::env;
use std::path::Path;
use std::process;

use lazy_static::initialize;
use inotify::{Inotify, WatchMask};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args : Vec<String> = env::args().collect();
    // 监听配置文件，如果改变就重启程序
    let yml_source = env::var("HOME").unwrap() + "/.config/rsblocks/rsblocks.yml";
    if !Path::new(&yml_source.as_str()).exists(){
        // 配置文件不存在，不用监听
        initialize(&config::CONFIG);
        // if X display is not found then exit the program
        if env::var("DISPLAY").is_err() {
            eprintln!("Error: No Display Running!");
            process::exit(1);
        };

        let blocks = BlockManager::new();
        run::run(blocks).await;
        return Ok(());
    }
    std::thread::spawn(move ||{
        let mut inotify = Inotify::init().expect("Error: can't init inotify!");
        inotify.add_watch(
            yml_source.as_str(),
            WatchMask::MODIFY
        ).expect("Error: can't add inotify watch on configure file!");
        loop {
            let mut buffer = [0;512];
            let events = inotify.read_events_blocking(&mut buffer);
            match events {
                Ok(evs) => {
                    for _ in evs {
                        // 配置文件改变，自动重启程序
                        println!("Config file change!");
                        let _ = exec::execvp("/proc/self/exe", &args);
                        process::exit(1);
                    }
                }
                _ => {}
            }
        }
    });
    // 开始运行
    initialize(&config::CONFIG);
    // if X display is not found then exit the program
    if env::var("DISPLAY").is_err() {
        eprintln!("Error: No Display Running!");
        process::exit(1);
    };

    let blocks = BlockManager::new();
    run::run(blocks).await;
    Ok(())
}