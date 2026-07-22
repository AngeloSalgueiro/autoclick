use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use std::{thread, time};

pub struct AutoclickModel {
    enigo: Enigo,
    frequency: time::Duration,
    is_active: bool,
    // input_type: ?,
    // start_key
}

impl Default for AutoclickModel {
    fn default() -> Self {
        Self {
            enigo: Enigo::new(&Settings::default()).unwrap(),
            frequency: time::Duration::from_millis(100),
            is_active: false,
        }
    }
}

impl AutoclickModel {
    pub fn input_for_x_time(&mut self) {
        self.is_active = true;

        while self.is_active {
            // Need error handling
            let _ = self.enigo.button(Button::Left, Click);
            thread::sleep(self.frequency);
        }
    }
}
