use nannou::{event::WindowEvent, color, App, Frame};

use crate::input::Input;
use crate::ui_elem::{Button, UiElem};

pub trait Draw {
    fn draw(&self, app:&App, frame: &Frame);
}


pub struct Ui {
    pub input: Input,
    content: Vec<UiElem>,
}

impl Ui {
    pub fn new() -> Ui {

        let input = Input::new();

        Ui {
            input,
            content: Vec::new()
        }
    }

    pub fn handle_input(&mut self, event: WindowEvent) {
        self.input.handle_input(event);
    }

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

    pub fn refresh(&mut self) {
        self.content = Vec::new();
    }

}

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