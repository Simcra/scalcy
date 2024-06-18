use std::sync::{Arc, Mutex};

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
    prev_tokenstream: Vec<Token>,
    curr_tokenstream: Vec<Token>,
}

fn flatten_tokenstream(tokenstream: Vec<Token>) -> String {
    let mut result = String::new();
    for token in tokenstream {
        result.push_str(String::from(token).as_str());
    }
    return result;
}

fn perform_calculation(app: CalculatorApp, state_ref: Arc<Mutex<CalculatorState>>) {
}

fn add_to_tokenstream(app: CalculatorApp, state_ref: Arc<Mutex<CalculatorState>>, token: Token) {
    let mut state = state_ref.lock().unwrap();
    let mut tokenstream = state.curr_tokenstream.clone();

    tokenstream.push(token);

    state.prev_tokenstream = state.curr_tokenstream.clone();
    state.curr_tokenstream = tokenstream.clone();
    app.set_value(flatten_tokenstream(tokenstream).into());
}

fn handle_button_press(app: CalculatorApp, state_ref: Arc<Mutex<CalculatorState>>, button: Button) {
    let token = Token::from(button);
    println!("Button: {} - {}", String::from(button), String::from(token));

    match token {
        Token::Equal => perform_calculation(app, state_ref),
        Token::Unknown => {}
        _ => add_to_tokenstream(app, state_ref, token),
    }
}

fn handle_key_press(app: CalculatorApp, state_ref: Arc<Mutex<CalculatorState>>, key: Key) {
    let token = Token::from(key);
    println!("Key: {} - {}", String::from(key), String::from(token));

    match token {
        Token::Equal => perform_calculation(app, state_ref),
        Token::Unknown => {}
        _ => add_to_tokenstream(app, state_ref, token),
    }
}

fn main() {
    let app = CalculatorApp::new().unwrap();
    let state = Arc::new(Mutex::new(CalculatorState::default()));

    app.global::<CalculatorLogic>().on_button_pressed({
        let app_weak = app.as_weak();
        let state_weak = Arc::clone(&state);
        move |value| {
            handle_button_press(
                app_weak.unwrap(),
                Arc::clone(&state_weak),
                Button::from(value.as_str()),
            )
        }
    });

    app.global::<CalculatorLogic>().on_key_pressed({
        let app_weak = app.as_weak();
        let state_weak = Arc::clone(&state);
        move |value: slint::SharedString| {
            handle_key_press(
                app_weak.unwrap(),
                Arc::clone(&state_weak),
                Key::from(value.as_str()),
            )
        }
    });

    app.run().unwrap();
}
