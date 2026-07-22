slint::include_modules!();

fn main() {
    // View
    let main_window = MainWindow::new().unwrap();
    // let weak_window = main_window.as_weak();

   
    main_window.run().unwrap();
}
