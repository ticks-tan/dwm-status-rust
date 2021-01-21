use crate::config::CONFIG;
use alsa::mixer::{Mixer, SelemChannelId, SelemId};

// getting volume percentage
pub fn get_volume() -> String {
    let card = if CONFIG.volume.card == "PULSE" {
        "pulse"
    } else {
        "default"
    };

    let mixer = Mixer::new(card, false).expect("Failed to open mixer");
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&selem_id).expect("Couldn't find selem");
    let selem_chan_id = SelemChannelId::FrontLeft;

    let (min, max) = selem.get_playback_volume_range();
    let mut raw_volume = selem
        .get_playback_volume(selem_chan_id)
        .expect("Failed to get raw_volume");

    let range = max - min;
    let vol = if range == 0 {
        0
    } else {
        raw_volume -= min;
        ((raw_volume as f64 / range as f64) * 100.) as u64
    };

    format!("  {}  {}%  {}", CONFIG.volume.icon, vol, CONFIG.seperator)
}
