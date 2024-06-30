const LABEL: &str = "";
const FORMAT: &str = "%I:%M%p";

pub fn clock() -> String {
    let mut output = String::new();

    if !LABEL.is_empty() {
        output.push_str(LABEL);
    }

    let currdt = chrono::Local::now();
    output.push_str(&currdt.format(FORMAT).to_string());

    output
}
