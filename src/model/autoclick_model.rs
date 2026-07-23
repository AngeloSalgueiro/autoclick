use enigo::{Button, Direction::Click, Enigo, Mouse, Settings};

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyManager,
    HotKeyState
};

pub struct AutoclickModel {
    frequency: Duration,
    active_state: Arc<AtomicBool>,
    task: Option<JoinHandle<()>>,
    // input_type: ?,
    toggle_key: String,
    hotkey_manager: GlobalHotKeyManager,
}

impl Default for AutoclickModel {
    fn default() -> Self {
        Self {
            frequency: Duration::from_millis(100),
            active_state: Arc::new(AtomicBool::new(false)),
            task: None,
            toggle_key: format!("F6"),
            hotkey_manager: GlobalHotKeyManager::new().unwrap(),
        }
    }
}

impl AutoclickModel {
    pub fn get_active_state(&self) -> bool {
        return self.active_state.load(Ordering::Relaxed);
    }

    pub fn start(&mut self) {
        if self.task.is_some() {
            self.stop();
            return;
        }

        let frequency = self.frequency;
        let running = self.active_state.clone();

        running.store(true, Ordering::Relaxed);

        self.task = Some(thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            while running.load(Ordering::Relaxed) {
                println!("click");
                let _ = enigo.button(Button::Left, Click);
                thread::sleep(frequency);
            }
        }));
    }

    pub fn stop(&mut self) {
        self.active_state.store(false, Ordering::Relaxed);

        if let Some(handle) = self.task.take() {
            let _ = handle.join(); // wait for clean exit
        }
    }

    pub fn detect_toggle_key(&self, model: Arc<Mutex<Self>>) {
        let hotkey = HotKey::new(None, Code::F5);

        self.hotkey_manager.register(hotkey).unwrap();

        std::thread::spawn(move || {
            while let Ok(event) = global_hotkey::GlobalHotKeyEvent::receiver().recv() {
                if event.state() == HotKeyState::Pressed {
                    if let Ok(mut model) = model.lock() {
                        model.start();
                    }
                }
            }
        });
    }
}
