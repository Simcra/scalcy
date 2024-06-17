use button::Button;
use key::Key;
use slint::Model;

mod button;
mod key;
mod operation;
mod token;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_button_clicked({
        let ui_handle = ui.as_weak();
        move |button_text| {
            let ui = ui_handle.unwrap();
            let button = Button::from(button_text.as_str());
            println!("Button: {}", String::from(button));
            // let token = Token::from_button(&button_text).unwrap();
            // let token_str: String = token.into();
            // ui.set_displayed_value(ui.get_displayed_value() + token_str.as_str());
        }
    });

    ui.on_key_pressed({
        let ui_handle = ui.as_weak();
        move |key_text: slint::SharedString| {
            let ui = ui_handle.unwrap();
            let key = Key::from(key_text.as_str());
            println!("Key: {}", String::from(key));
            // let token = Token::from_key(&key_text);
            // if !token.is_none() {
            // ui.set_display_value(token.unwrap().to_string().into());
            // }
        }
    });

    ui.on_flatten_tokenstream(move |tokenstream: slint::ModelRc<slint::SharedString>| {
        let mut result: String = "".to_owned();
        for token in tokenstream.iter() {
            result.push_str(token.as_str());
        }

        return result.into();
    });

    ui.run()
}
