use sola_raylib::RaylibHandle;

pub struct Timer {
    pub duration: f64,
    pub start_time: f64,
    pub active: bool,
    pub repeat: bool,
    pub func: Option<Box<dyn FnMut()>>,
}

impl Timer {
    // Equivalent to __init__
    pub fn new(
        rl: &RaylibHandle,
        duration: f64,
        repeat: bool,
        autostart: bool,
        func: Option<Box<dyn FnMut()>>,
    ) -> Self {
        let mut timer = Self {
            duration,
            start_time: 0.0,
            active: false,
            repeat,
            func,
        };

        if autostart {
            timer.activate(rl);
        }

        timer
    }

    // Equivalent to activate(self)
    pub fn activate(&mut self, rl: &RaylibHandle) {
        self.active = true;
        self.start_time = rl.get_time();
    }

    // Equivalent to deactivate(self)
    pub fn deactivate(&mut self, rl: &RaylibHandle) {
        self.active = false;
        self.start_time = 0.0;
        if self.repeat {
            self.activate(rl);
        }
    }

    // Equivalent to update(self)
    pub fn update(&mut self, rl: &RaylibHandle) {
        if self.active && rl.get_time() - self.start_time >= self.duration {
            if let Some(ref mut callback) = self.func {
                callback();
            }
            self.deactivate(rl);
        }
    }
}
