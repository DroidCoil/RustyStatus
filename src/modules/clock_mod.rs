const LABEL: &str = "";
const FORMAT: &str = "%I:%M%p";

pub fn clock() -> String {
    let mut output: String = String::from("");

    // Add label if not blank
    if LABEL.len() > 0 {
        output.push_str(LABEL);
    }

    // Add time in format
    let currdt = chrono::Local::now();
    output.push_str(&currdt.format(FORMAT).to_string());
    //let time = currdt.format(FORMAT);

    return output;
}
