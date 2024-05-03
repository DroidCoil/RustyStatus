mod config;
mod modules;

use std::{process::Command, thread::sleep};

use config::{configs, Module};

fn main() {
    // Pull all active modules
    let mut barmods: Vec<Module> = configs();
    let mut timer: i32 = 1;
    let mut statusid: i32 = 0;

    // // Get min and max timer
    let mut vectimer: Vec<i32> = vec![];
    let maxtime: i32;
    for m in &barmods {
        vectimer.push(m.timer);
    }
    maxtime = vectimer.into_iter().max().unwrap();

    for m in &mut barmods {
        m._refresh();
        statusupdate(statusid, &m.output);
        statusid += 1;
    }

    // Bar refresh loop
    loop {
        sleep(std::time::Duration::from_secs(1));
        statusid = 0;
        if timer > maxtime {
            timer = 1;
        }
        for m in &mut barmods {
            if timer % m.timer == 0 {
                m._refresh();
                statusupdate(statusid, &m.output);
            }
            statusid += 1;
        }
        timer += 1;
    }
}

fn statusupdate(statusid: i32, out: &str) {
    Command::new("duskc")
        .arg("--ignore-reply")
        .arg("run_command")
        .arg("setstatus")
        .arg(statusid.to_string().as_str())
        .arg(out)
        .output()
        .expect("failed to execute process");
}
