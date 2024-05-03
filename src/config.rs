use crate::modules::clock_mod::clock;

pub fn configs() -> Vec<Module> {
    let mut mods: Vec<Module> = vec![];
    mods = pushmod(mods, 0, 60, clock);
    //mods = pushmod(mods, 1, 1, _batterystat);

    return mods;
}

pub struct Module {
    pub timer: i32,
    pub function: Box<dyn Fn() -> String>,
    pub output: String,
    pub statusid: i32,
}

impl Module {
    pub fn _refresh(&mut self) {
        let _temp = &self.function;
        self.output = _temp();
    }
}

fn pushmod(
    mut mv: Vec<Module>,
    statusid: i32,
    timer: i32,
    fun: impl Fn() -> String + 'static,
) -> Vec<Module> {
    mv.push(Module {
        timer, // Set how often you want this to run
        statusid,
        function: Box::new(fun),
        output: String::from(""), // Module Name
    });
    return mv;
}
