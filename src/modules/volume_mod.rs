use alsa::{
    mixer::{SelemChannelId, SelemId},
    Mixer,
};
use anyhow::{anyhow, Result};

const LABEL: &str = "Vol:";

pub fn volumestat() -> Result<String> {
    let mut output = String::from(LABEL);

    let mixer = Mixer::new("default", false)?;
    let selem_id = SelemId::new("Master", 0);
    let elem = mixer
        .find_selem(&selem_id)
        .ok_or_else(|| anyhow!("Unable to find Master element"))?;
    let (_min_volume, max_volume) = elem.get_playback_volume_range();
    let volume = elem.get_playback_volume(SelemChannelId::FrontLeft)?;
    let is_muted = elem.get_playback_switch(SelemChannelId::FrontLeft)? != 1;

    if is_muted {
        output.push_str("MUTED");
    } else {
        let volume_percentage = ((volume * 100) as f64 / max_volume as f64).ceil() as i32;
        output.push_str(&format!("{}%", volume_percentage));
    }

    Ok(output)
}
