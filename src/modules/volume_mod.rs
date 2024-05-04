use std::cmp::max;

const LABEL: &str = "Vol:";

pub fn volumestat() -> String {
    let mut output: String = String::from("");

    let mixer = alsa::Mixer::new("default", false).unwrap();
    let selem_id = alsa::mixer::SelemId::new("Master", 0);
    let elem = mixer.find_selem(&selem_id).unwrap();
    let volume_range = elem.get_playback_volume_range();
    let volume = elem
        .get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft)
        .unwrap();
    let mute = elem
        .get_playback_switch(alsa::mixer::SelemChannelId::FrontLeft)
        .unwrap();

    output.push_str(LABEL);

    // check if muted
    if mute != 1 {
        output.push_str("MUTED");
    } else {
        // get volume and convert it
        let volume_h = (volume * 100) as f64 / max(volume_range.0, volume_range.1) as f64;
        output.push_str(&volume_h.ceil().to_string());
        output.push_str("%")
    }
    return output;
}
