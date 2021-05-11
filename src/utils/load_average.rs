use crate::config::CONFIG;
use crate::types::ThreadsData;
use nix::sys::sysinfo;

pub fn get_load_avg() -> ThreadsData {
    let load = sysinfo::sysinfo().unwrap().load_average().0;
    let data = format!(
        "  {}  {:.2}  {}",
        CONFIG.loadavg.icon, load, CONFIG.seperator
    );
    ThreadsData::LoadAvg(data)
}
