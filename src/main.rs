mod calculator;
mod parsetree;

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use calculator::*;
use parsetree::*;
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

fn handle_key_press(ui: MainWindow, calculator: &mut RefMut<Calculator>, value: &str) {
    match value {
        "\x08" => {
            // BACKSPACE key
            calculator.pop();
            calculator.calculate();
        }
        "\x7f" => {
            // DELETE key
            calculator.clear();
            calculator.calculate();
        }
        "\n" => calculator.store(),   // ENTER key (LF)
        "\r" => calculator.store(),   // ENTER key (CR)
        "\r\n" => calculator.store(), // ENTER key (CR+LF)
        _ => {
            calculator.append(value);
            calculator.calculate();
        }
    };

    ui.set_input(SharedString::from(calculator.input()));
    ui.set_history(ModelRc::new(VecModel::from(
        calculator
            .history()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<SharedString>>(),
    )));
    ui.set_result(SharedString::from(calculator.result()));
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let calculator = Rc::new(RefCell::new(Calculator::default()));

    ui.global::<CalcLogic>()
        .on_is_empty(|value| value.is_empty());

    ui.global::<CalcLogic>().on_key_pressed({
        let ui_weak = ui.as_weak();
        let calculator_weak = calculator.clone();
        move |value| {
            handle_key_press(
                ui_weak.unwrap(),
                &mut calculator_weak.borrow_mut(),
                value.as_str(),
            )
        }
    });

    ui.run()
}
