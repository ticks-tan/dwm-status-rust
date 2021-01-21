use crate::config::CONFIG;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::{arg, blocking::Connection};
use std::time::Duration;

// getting spotify current artist and title.
// FIXME: I know im lazy asshole, this error handling looks ugly, i dont like it too, need to fix soon.
pub fn get_spotify() -> String {
    let conn = match Connection::new_session() {
        Ok(conn) => conn,
        _ => return String::from(""),
    };

    let p = conn.with_proxy(
        "org.mpris.MediaPlayer2.spotify",
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );

    let metadata: arg::PropMap = match p.get("org.mpris.MediaPlayer2.Player", "Metadata") {
        Ok(data) => data,
        _ => return String::from(""),
    };

    let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
    let artist: Option<&Vec<String>> = arg::prop_cast(&metadata, "xesam:artist");

    let title = match title {
        Some(title) => title,
        _ => "",
    };

    let artist = match artist {
        Some(artist_vec) => match artist_vec.first() {
            Some(name) => name,
            _ => "",
        },
        None => "",
    };

    format!(
        "  {}  {} - {}  {}",
        CONFIG.spotify.icon, artist, title, CONFIG.seperator
    )
}
