use nannou::{event::WindowEvent, color, App, Frame};

use crate::input::Input;
use crate::ui_elem::{Button, UiElem};

/// Draw trait for ui and ui elements
pub trait Draw {
    fn draw(&self, app:&App, frame: &Frame);
}

/// Ui struct contains and handles input and ui
pub struct Ui {
    pub input: Input,
    content: Vec<UiElem>,
}

impl Ui {
    /// create new ui
    pub fn new() -> Ui {

        let input = Input::new();

        Ui {
            input,
            content: Vec::new()
        }
    }
    /// handle input by calling Input struct from ui
    pub fn handle_input(&mut self, event: WindowEvent) {
        self.input.handle_input(event);
    }
    /// add Ui element to ui and returns ui element to call funktions on, example: .is_clicked()
    pub fn add(&mut self, mut ui_elem: UiElem) -> UiElem {
        
        match ui_elem {
            UiElem::Button(ref mut button) => {
                button.update(&self.input);
            }
            UiElem::Label(ref label) => {

            }
        }
        self.content.push(ui_elem.clone());
        ui_elem
    }
    /// overwrites content of ui.content with new vec (clear data)
    pub fn refresh(&mut self) {
        self.content = Vec::new();
    }

}
/// draw trait for ui struct
impl Draw for Ui {
    fn draw(&self, app:&App, frame: &Frame) {
        self.content.iter().for_each(|elem| {
            match elem {
                UiElem::Button(x) => x.draw(app, frame),
                UiElem::Label(x) => {x.draw(app, frame)},
            }
        });
    }
}