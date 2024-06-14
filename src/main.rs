slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_button_clicked({
        let ui_handle = ui.as_weak();
        move |value| {
            let ui = ui_handle.unwrap();
            ui.set_display_value(value);
        }
    });

    ui.on_key_pressed({
        let ui_handle = ui.as_weak();
        move |value| {
            let ui = ui_handle.unwrap();
            ui.set_display_value(value);
        }
    });

    ui.run()
}
