use token::Token;

mod token;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_button_clicked({
        let ui_handle = ui.as_weak();
        move |value| {
            let ui = ui_handle.unwrap();
            let token = Token::from_button(&value).unwrap();
            ui.set_display_value(token.to_string().into());
        }
    });

    ui.on_key_pressed({
        let ui_handle = ui.as_weak();
        move |value| {
            let ui = ui_handle.unwrap();
            let token = Token::from_keypress(&value);
            if !token.is_none() {
                ui.set_display_value(token.unwrap().to_string().into());
            }
        }
    });

    ui.run()
}
