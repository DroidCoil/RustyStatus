use battery::{Manager, State};

const LABEL: &str = "Bat:";
const CHARGING: &str = "+";
const DISCHARGE: &str = "-";
const FULL_THRESHOLD: u32 = 95;
const ADDED_ZERO: bool = true;

pub fn batterystat() -> String {
    let mut output = String::from(LABEL);

    let manager = Manager::new().unwrap();
    let battery = match manager.batteries().unwrap().next() {
        Some(Ok(bat)) => bat,
        _ => return "Unable to access battery information".to_string(),
    };

    let percentage = (battery.state_of_charge().value * 100.0) as u32;

    if percentage < FULL_THRESHOLD {
        match battery.state() {
            State::Charging => output.push_str(CHARGING),
            State::Discharging => output.push_str(DISCHARGE),
            _ => (),
        }
    }

    output.push_str(&format!("{}%", percentage));

    if let Some(time) = battery.time_to_full().or(battery.time_to_empty()) {
        let mut hours = (time.value as u32 / 3600).to_string();
        let mut minutes = ((time.value as u32 % 3600) / 60).to_string();
        if ADDED_ZERO == true {
            if hours.len() == 1 {
                hours.insert(0, '0')
            }
            if minutes.len() == 1 {
                minutes.insert(0, '0')
            }
        }
        output.push_str(&format!(" ({:02}:{:02})", hours, minutes));
    }

    output
}
