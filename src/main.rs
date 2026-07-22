use crate::model::autoclick_model::{self, AutoclickModel};

pub mod model;

slint::include_modules!();

fn main() {
    // View
    let main_window = MainWindow::new().unwrap();
    // let weak_window = main_window.as_weak();
    let mut autoclick_model: AutoclickModel = AutoclickModel::default();

    main_window.on_start(move |starting: bool|{
        if starting{
            autoclick_model.start();
        } else{
            autoclick_model.stop();
        }

    });

    main_window.run().unwrap();
}
