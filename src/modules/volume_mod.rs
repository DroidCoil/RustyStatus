use alsa::{mixer::{SelemChannelId, SelemId}, Mixer};

const LABEL: &str = "Vol:";

pub fn volumestat() -> String {
    let mut output = String::from(LABEL);

    let mixer = Mixer::new("default", false).unwrap();
    let selem_id = SelemId::new("Master", 0);
    let elem = mixer.find_selem(&selem_id).unwrap();
    let (min_volume, max_volume) = elem.get_playback_volume_range();
    let volume = elem.get_playback_volume(SelemChannelId::FrontLeft).unwrap();
    let is_muted = elem.get_playback_switch(SelemChannelId::FrontLeft).unwrap() != 1;

    if is_muted {
        output.push_str("MUTED");
    } else {
        let volume_percentage = ((volume * 100) as f64 / std::cmp::max(min_volume, max_volume) as f64).ceil() as i32;
        output.push_str(&format!("{}%", volume_percentage));
    }

    output
}
