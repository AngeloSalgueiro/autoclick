use std::sync::{Arc, Mutex};

use crate::model::autoclick_model::AutoclickModel;

pub mod model;

slint::include_modules!();

fn main() {
    // View
    let main_window = MainWindow::new().unwrap();
    // let weak_window = main_window.as_weak();
    let autoclick_model = Arc::new(Mutex::new(AutoclickModel::default()));

    if let Ok(model) = autoclick_model.lock() {
        model.detect_toggle_key(Arc::clone(&autoclick_model));
    }

    main_window.on_start(move |starting: bool| {
        if let Ok(mut autoclick_model) = autoclick_model.lock() {
            if starting {
                autoclick_model.start();
            } else {
                autoclick_model.stop();
            }
        }
    });

    main_window.run().unwrap();
}
