use battery::State;

extern crate battery;

const LABEL: &str = "Bat:";
const CHARGING: &str = "+";
const FULL: &str = "";
const DISCHARGE: &str = "-";
const FULLTHRESHOLD: u32 = 95;
const NOTIFICATIONS: bool = true;

pub fn _batterystat() -> String {
    let mut output: String = String::from("");

    let batman = battery::Manager::new();
    let _battery = match batman.unwrap().batteries().unwrap().next() {
        Some(Ok(battery)) => battery,
        Some(Err(_e)) => {
            return "Unable to access battery information".to_string();
        }
        None => {
            return "Unable to find any batteries".to_string();
        }
    };

    // Get battery state
    let state = _battery.state();

    // Get Charge Percantage
    let percentage = (_battery.state_of_charge().value * 100.0) as u32;

    // Hours of charge remaining
    let mut hours: String = String::new();

    // Minutes of charge remaining
    let mut minutes: String = String::new();

    if state == State::Discharging || state == State::Charging {
        if _battery.time_to_full().is_some() {
            let time = _battery.time_to_full().unwrap().value as u32;
            hours = (time / 3600 as u32).to_string();
            minutes = ((time % 3600) / 60 as u32).to_string();
        } else if _battery.time_to_empty().is_some() {
            let time = _battery.time_to_empty().unwrap().value as u32;
            hours = (time / 3600 as u32).to_string();
            minutes = ((time % 3600) / 60 as u32).to_string();
        }

        output.push_str(LABEL);

        let outstate: &str;

        match state {
            State::Charging => outstate = CHARGING,
            State::Discharging => outstate = DISCHARGE,
            State::Empty => outstate = "",
            State::Full => outstate = FULL,
            State::Unknown => outstate = "?",
            _ => outstate = "missing",
        }

        if percentage >= FULLTHRESHOLD {
            if _battery.time_to_empty().is_some() {
                output.push_str(DISCHARGE);
            }
            output.push_str("100");
            output.push_str("%");
        } else {
            output.push_str(outstate);
            output.push_str(percentage.to_string().as_str());
            output.push_str("%");
        }

        if hours.len() > 0 && minutes.len() > 0 {
            output.push_str(" (");
            if hours.len() < 2 {
                output.push_str("0");
            }
            output.push_str(hours.to_string().as_str());
            output.push_str(":");
            if minutes.len() < 2 {
                output.push_str("0");
            }
            output.push_str(minutes.to_string().as_str());
            output.push_str(")");
        }
    }

    return output;
}
