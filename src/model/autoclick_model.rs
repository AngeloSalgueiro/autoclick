use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct AutoclickModel {
    frequency: Duration,
    is_active: Arc<AtomicBool>,
    task: Option<JoinHandle<()>>,
    // input_type: ?,
    // start_key
}

impl Default for AutoclickModel {
    fn default() -> Self {
        Self {
            frequency: Duration::from_millis(100),
            is_active: Arc::new(AtomicBool::new(false)),
            task: None,
        }
    }
}

impl AutoclickModel {
    pub fn start(&mut self) {
        if self.task.is_some() {
            return;
        }

        let frequency = self.frequency;
        let running = self.is_active.clone();

        running.store(true, Ordering::Relaxed);

        self.task = Some(thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            while running.load(Ordering::Relaxed) {
                println!("test3");
                let _ = enigo.button(Button::Left, Click);
                thread::sleep(frequency);
                println!("{}", running.load(Ordering::Relaxed));
            }
        }));
    }

    pub fn stop(&mut self) {
        self.is_active.store(false, Ordering::Relaxed);

        if let Some(handle) = self.task.take() {
            let _ = handle.join(); // wait for clean exit
        }
    }
}
