use crate::config::ModuleConfig;

const LABEL: &str = "";
const FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn clock(conf: &ModuleConfig) -> String {
    let mut output = String::new();

    output.push_str(LABEL);

    let currdt = chrono::Local::now();
    output.push_str(&currdt.format(FORMAT).to_string());

    output
}
