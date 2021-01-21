use crate::config::CONFIG;
use mpd::{Client, Song};

// yes, error handling looks fucking sucks!
// getting mpd song file
pub fn get_mpd_current() -> String {
    let stream_path = format!("{}:{}", CONFIG.mpd.host, CONFIG.mpd.port);
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
        CONFIG.mpd.icon, current.file, CONFIG.seperator
    );

    result
}
