use sysinfo::{RefreshKind, System};
use anyhow::Result;

const LABEL: &str = "Mem:";

pub fn ramstat() -> Result<String> {
    let mut output = String::from(LABEL);

    let mut sys = System::new_with_specifics(RefreshKind::new());
    sys.refresh_memory();

    let used_mem = sys.used_memory() as f64 / (1024.0 * 1024.0); // Convert to MB
    let total_mem = sys.total_memory() as f64 / (1024.0 * 1024.0); // Convert to MB

    if total_mem >= 1024.0 {
        output.push_str(&format!(
            " {:.1}GB/{:.1}GB",
            used_mem / 1024.0, // Convert to GB
            total_mem / 1024.0 // Convert to GB
        ));
    } else if total_mem >= 1024.0 && used_mem < 1024.0 {
        output.push_str(&format!(
            " {:.1}MB/{:.1}GB",
            used_mem,
            total_mem / 1024.0 // Convert to GB
        ));
    } else {
        output.push_str(&format!(
            " {:.1}MB/{:.1}MB",
            used_mem,
            total_mem
        ));
    }

    Ok(output)
}
