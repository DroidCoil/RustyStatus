use crate::modules::{battery_mod::batterystat, clock_mod::clock, ram_mod::ramstat, volume_mod::volumestat};

pub fn configs() -> Vec<Module> {
    vec![
        create_module(60, clock),
        create_module(1, batterystat),
        create_module(1, volumestat),
        create_module(1, ramstat),
    ]
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
