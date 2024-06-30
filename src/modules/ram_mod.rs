use sysinfo::{RefreshKind, System};

const LABEL: &str = "Mem:";

pub fn ramstat() -> String {
    let mut output = String::from(LABEL);

    let used_mem: u64;
    let total_mem: u64;

    let mut sys = System::new_with_specifics(RefreshKind::new());
    sys.refresh_memory();

    if sys.used_memory() >= (1024 * 1024 * 1024) {
        used_mem = sys.used_memory() / (1024 * 1024 * 1024) as u64;
        output.push_str(&format!(" {}GB", used_mem));
    } else {
        used_mem = sys.used_memory() / (1024 * 1024) as u64;
        output.push_str(&format!(" {}MB", used_mem));
    }
    if sys.total_memory() >= (1024 * 1024 * 1024) {
        total_mem = sys.total_memory() / (1024 * 1024 * 1024) as u64;
        output.push_str(&format!("/{}GB", total_mem));
    } else {
        total_mem = sys.total_memory() / (1024 * 1024) as u64;
        output.push_str(&format!("/{}MB", total_mem));
    }

    return output;
}
