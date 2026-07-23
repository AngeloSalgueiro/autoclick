use std::sync::{Arc, Mutex};

use crate::model::autoclick_model::{self, AutoclickModel};

pub mod model;

slint::include_modules!();

fn main() {
    // Views
    let main_window = MainWindow::new().unwrap();
    let weak_window = main_window.as_weak();

    // Models
    let autoclick_model = Arc::new(Mutex::new(AutoclickModel::default()));
    let autoclick_model_frequency = Arc::clone(&autoclick_model);
    

    if let Ok(model) = autoclick_model.lock() {
        model.detect_toggle_key(Arc::clone(&autoclick_model));
    }


    main_window.on_start_autoclick(move |starting: bool| {
        if let Ok(mut autoclick_model) = autoclick_model.lock() {
            if starting {
                autoclick_model.start();
            } else {
                autoclick_model.stop();
            }
        }
    });

    main_window.on_update_frequency(move || {
        let ui = weak_window.unwrap();
        let new_frequency = ui.get_frequency_value();

        let new_frequency = match new_frequency.parse::<u64>(){
            Ok(value) => value,
            Err(_) => 0
        };

        if let Ok(mut autoclick_model) = autoclick_model_frequency.lock() {
            autoclick_model.set_frequency(new_frequency);
        }
    });

    main_window.run().unwrap();
}
