pub struct Timer {
    duration: f32,
    elapsed: f32,
    active: bool,
    repeat: bool,
}

impl Timer {
    pub fn new(duration: f32, repeat: bool, autostart: bool) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            active: autostart,
            repeat,
        }
    }

    /// Advances the timer by `dt` seconds.
    ///
    /// Returns `true` on a tick that reaches the timer's duration.
    pub fn tick(&mut self, dt: f32) -> bool {
        if !self.active {
            return false;
        }

        self.elapsed += dt;
        if self.elapsed < self.duration {
            return false;
        }

        self.elapsed -= self.duration;
        if !self.repeat {
            self.active = false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    #[test]
    fn fires_once_when_not_repeating() {
        let mut timer = Timer::new(0.5, false, true);

        assert!(!timer.tick(0.4));
        assert!(timer.tick(0.2));
        assert!(!timer.tick(1.0));
    }

    #[test]
    fn fires_repeatedly_when_repeating() {
        let mut timer = Timer::new(0.5, true, true);

        assert!(!timer.tick(0.4));
        assert!(timer.tick(0.2));
        assert!(!timer.tick(0.2));
        assert!(timer.tick(0.2));
    }
}
