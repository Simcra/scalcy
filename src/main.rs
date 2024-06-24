use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

slint::include_modules!();

#[derive(Default)]
struct CalculatorState {
    current_tokenstream: slint::SharedString,
    previous_tokenstream: slint::SharedString,
    result: Option<f64>,
}

fn update_display(app: CalculatorApp, state: &mut RefMut<CalculatorState>) {
    app.set_result(match state.result {
        Some(result) => result.to_string().into(),
        None => "".into(),
    });
    app.set_tokenstream(state.current_tokenstream.clone());
}

fn do_backspace(app: CalculatorApp, state: &mut RefMut<CalculatorState>) {
    if state.current_tokenstream.len() > 0 {
        state.previous_tokenstream = state.current_tokenstream.clone();
        let tokenstream = state.current_tokenstream.as_str();
        state.current_tokenstream = tokenstream[..tokenstream.len() - 1].into();
    }
    update_display(app, state);
}

fn do_clear(app: CalculatorApp, state: &mut RefMut<CalculatorState>) {
    state.previous_tokenstream = slint::SharedString::default();
    state.current_tokenstream = slint::SharedString::default();
    update_display(app, state);
}

fn do_equals(app: CalculatorApp, state: &mut RefMut<CalculatorState>) {
    state.previous_tokenstream = state.current_tokenstream.clone();
    state.current_tokenstream = match state.result {
        Some(result) => result.to_string().into(),
        None => "Err".into(),
    };
    update_display(app, state);
}

fn do_add_token(app: CalculatorApp, state: &mut RefMut<CalculatorState>, value: &str) {
    state.previous_tokenstream = state.current_tokenstream.clone();
    state.current_tokenstream.push_str(value);
    update_display(app, state);
}

fn handle_button_press(app: CalculatorApp, state: &mut RefMut<CalculatorState>, value: &str) {
    match value {
        "C" => do_clear(app, state),          // Do clear,
        "=" => do_equals(app, state), // Store result into tokenstream if valid - otherwise error,
        _ => do_add_token(app, state, value), // Add the token to the tokenstream
    };
}

fn handle_key_press(app: CalculatorApp, state: &mut RefMut<CalculatorState>, value: &str) {
    match value {
        "\x08" => do_backspace(app, state), // Do backspace, BACKSPACE key
        "\x7f" => do_clear(app, state),     // Do clear, DELETE key
        "\n" => do_equals(app, state), // Store result in tokenstream if valid - otherwise error,
        "\r" => do_equals(app, state), // Store result in tokenstream if valid - otherwise error,
        "\r\n" => do_equals(app, state), // Store result in tokenstream if valid - otherwise error,
        _ => do_add_token(app, state, value),
    };
}

fn main() {
    let app = CalculatorApp::new().unwrap();
    let state = Rc::new(RefCell::new(CalculatorState::default()));

    app.global::<CalculatorLogic>().on_button_pressed({
        let app_weak = app.as_weak();
        let state_weak = state.clone();
        move |value| {
            handle_button_press(
                app_weak.unwrap(),
                &mut state_weak.borrow_mut(),
                value.as_str(),
            )
        }
    });

    app.global::<CalculatorLogic>().on_key_pressed({
        let app_weak = app.as_weak();
        let state_weak = state.clone();
        move |value| {
            handle_key_press(
                app_weak.unwrap(),
                &mut state_weak.borrow_mut(),
                value.as_str(),
            )
        }
    });

    app.run().unwrap();
}
