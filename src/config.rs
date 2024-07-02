use crate::modules::{
    battery_mod::batterystat, clock_mod::clock, ram_mod::ramstat, volume_mod::volumestat,
};
use anyhow::Result;

pub fn configs() -> Result<Vec<Module>> {
    Ok(vec![
        create_module(clock),
        create_module(ramstat),
        //create_module(|_| batterystat().unwrap_or_else(|e| format!("Battery error: {}", e))),
        //create_module(|_| volumestat().unwrap_or_else(|e| format!("Volume error: {}", e))),
        //create_module(|_| ramstat().unwrap_or_else(|e| format!("RAM error: {}", e))),
    ])
}

pub struct ModuleConfig {
    pub icons: bool,
    pub color: String,
    pub label: String,
    pub timer: i32,
}

pub struct Module {
    pub config: ModuleConfig,
    pub function: Box<dyn Fn(&ModuleConfig) -> String>,
    pub output: String,
}

impl Module {
    pub fn refresh(&mut self) {
        self.output = (self.function)(&self.config);
    }
}

fn create_module(fun: impl Fn(&ModuleConfig) -> String + 'static) -> Module {
    let modconfig = ModuleConfig {
        icons: true,
        color: "".to_string(),
        label: "".to_string(),
        timer: 1,
    };
    Module {
        config: modconfig,
        function: Box::new(fun),
        output: String::new(),
    }
}
