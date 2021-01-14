use crate::types::Config;
use mpd::{Client, Song};

// yes, error handling looks fucking sucks!
// getting mpd song file
pub fn get_mpd_current(config: &Config) -> String {
    let stream_path = format!("{}:{}", config.mpd.host, config.mpd.port);
    let empty_string = String::from("");
    let mut conn = match Client::connect(&stream_path) {
        Ok(connection) => connection,
        _ => return empty_string,
    };
    let current: Song = match conn.currentsong() {
        Ok(opt) => match opt {
            Some(song) => song,
            _ => return empty_string,
        },
        _ => return empty_string,
    };

    let result = format!(
        "  {}  {}  {}",
        config.mpd.icon, current.file, config.seperator
    );

    result
}
