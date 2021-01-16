use crate::types::Config;

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
