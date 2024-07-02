mod config;
mod modules;

use anyhow::Result;
use config::configs;
use std::{process::Command, thread::sleep, time::Duration};

fn main() -> Result<()> {
    let mut barmods = configs()?;
    let maxtime = barmods.iter().map(|m| m.timer).max().unwrap_or(1);
    let mut timer = 1;

    // Initial status update
    for (statusid, m) in barmods.iter_mut().enumerate() {
        m.refresh();
        statusupdate(statusid as i32, &m.output)?;
    }

    loop {
        sleep(Duration::from_secs(1));
        if timer > maxtime {
            timer = 1;
        }

        for (statusid, m) in barmods.iter_mut().enumerate() {
            if timer % m.timer == 0 {
                m.refresh();
                statusupdate(statusid as i32, &m.output)?;
            }
        }
        timer += 1;
    }
}

fn statusupdate(statusid: i32, out: &str) -> Result<()> {
    Command::new("duskc")
        .args(&[
            "--ignore-reply",
            "run_command",
            "setstatus",
            &statusid.to_string(),
            out,
        ])
        .output()?;
    Ok(())
}
