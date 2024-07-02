use crate::modules::{
    battery_mod::batterystat, clock_mod::clock, ram_mod::ramstat, volume_mod::volumestat,
};
use anyhow::Result;

pub fn configs() -> Result<Vec<Module>> {
    Ok(vec![
        create_module(60, clock),
        create_module(1, || {
            batterystat().unwrap_or_else(|e| format!("Battery error: {}", e))
        }),
        create_module(1, || {
            volumestat().unwrap_or_else(|e| format!("Volume error: {}", e))
        }),
        create_module(1, || {
            ramstat().unwrap_or_else(|e| format!("RAM error: {}", e))
        }),
    ])
}

pub struct Module {
    pub timer: i32,
    pub function: Box<dyn Fn() -> String>,
    pub output: String,
}

impl Module {
    pub fn refresh(&mut self) {
        self.output = (self.function)();
    }
}

fn create_module(timer: i32, fun: impl Fn() -> String + 'static) -> Module {
    Module {
        timer,
        function: Box::new(fun),
        output: String::new(),
    }
}
