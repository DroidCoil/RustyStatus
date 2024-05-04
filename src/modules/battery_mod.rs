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

    // select battery
    let batterymanager = battery::Manager::new();
    let battery = match batterymanager.unwrap().batteries().unwrap().next() {
        Some(Ok(battery)) => battery,
        Some(Err(_e)) => {
            return "Unable to access battery information".to_string();
        }
        None => {
            return "Unable to find any batteries".to_string();
        }
    };

    // add label
    output.push_str(LABEL);

    // Get percentage
    let percentage = (battery.state_of_charge().value * 100.0) as u32;

    // Get charge state of battery
    if percentage >= FULLTHRESHOLD && battery.state() == State::Charging {
        output.push_str("")
    } else {
        match battery.state() {
            State::Charging => output.push_str(CHARGING),
            State::Discharging => output.push_str(DISCHARGE),
            State::Empty => output.push_str(""),
            State::Full => output.push_str(FULL),
            State::Unknown => output.push_str("?"),
            _ => output.push_str("missing"),
        }
    }

    // Add percentage
    output.push_str(&percentage.to_string());
    output.push_str("%");

    if battery.time_to_full().is_some() || battery.time_to_empty().is_some() {
        let hours: String;
        let minutes: String;
        let time: u32;
        if battery.time_to_full().is_some() {
            time = battery.time_to_full().unwrap().value as u32;
        } else {
            time = battery.time_to_empty().unwrap().value as u32;
        }

        hours = (time / 3600 as u32).to_string();
        minutes = ((time % 3600) / 60 as u32).to_string();

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
