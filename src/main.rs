use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use button::Button;
use key::Key;
use operation::Operation;
use token::Token;

mod button;
mod key;
mod operation;
mod token;

slint::include_modules!();

#[derive(Default)]
struct CalculatorState {
    previous_tokenstream: Vec<Token>,
    current_tokenstream: Vec<Token>,
}

fn main() {
    let app = CalculatorApp::new().unwrap();
    let state = Rc::new(RefCell::new(CalculatorState::default()));

    app.global::<CalculatorLogic>().on_button_pressed({
        let weak = app.as_weak();
        let state = state.as_ptr();
        move |button_text| {
            let app = weak.unwrap();
            let button = Button::from(button_text.as_str());
            let token = Token::from(button);
            println!("Button: {} - {}", String::from(button), String::from(token));
        }
    });

    app.global::<CalculatorLogic>().on_key_pressed({
        let weak = app.as_weak();
        let state = state.as_ptr();
        move |key_text| {
            let app = weak.unwrap();
            let key = Key::from(key_text.as_str());
            let token = Token::from(key);
            println!("Key: {} - {}", String::from(key), String::from(token));
        }
    });

    app.run().unwrap();
}
