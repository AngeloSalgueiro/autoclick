use enigo::{Button, Direction::Click, Enigo, Mouse, Settings};

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use global_hotkey::{
    GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey},
};

pub struct AutoclickModel {
    frequency: Duration,
    running: Arc<AtomicBool>,
    task: Option<JoinHandle<()>>,
    // input_type: ?,
    toggle_key: String,
    hotkey_manager: GlobalHotKeyManager,
}

impl Default for AutoclickModel {
    fn default() -> Self {
        Self {
            frequency: Duration::from_millis(100),
            running: Arc::new(AtomicBool::new(false)),
            task: None,
            toggle_key: format!("F6"),
            hotkey_manager: GlobalHotKeyManager::new().unwrap(),
        }
    }
}

impl AutoclickModel {
    pub fn set_frequency(&mut self, new_frequency: u64) {
        println!("frequency chanded to {} ms", new_frequency);
        self.frequency = Duration::from_millis(new_frequency);
    }

    pub fn start(&mut self) {
        if self.task.is_some() {
            return;
        }

        let frequency = self.frequency;
        let running = self.running.clone();

        running.store(true, Ordering::Release);

        println!("starting");

        let handle = thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();

            while running.load(Ordering::Acquire) {
                let _ = enigo.button(Button::Left, Click);
                thread::park_timeout(frequency);
            }
        });

        self.task = Some(handle);
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Release);

        println!("stopping");

        if let Some(handle) = self.task.take() {
            handle.thread().unpark();
            handle.join().unwrap();
        }
    }

    pub fn detect_toggle_key(&self, model: Arc<Mutex<Self>>) {
        let hotkey = HotKey::new(None, Code::F6);

        self.hotkey_manager.register(hotkey).unwrap();

        std::thread::spawn(move || {
            while let Ok(event) = global_hotkey::GlobalHotKeyEvent::receiver().recv() {
                if event.state() == HotKeyState::Pressed {
                    if let Ok(mut model) = model.lock() {
                        if model.running.load(Ordering::Relaxed) {
                            model.stop();
                        } else {
                            model.start();
                        }
                    }
                }
            }
        });
    }
}
