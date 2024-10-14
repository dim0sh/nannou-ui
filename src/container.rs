use crate::{main, ui::Ui};
use nannou::prelude::*;
use crate::ui::Draw;
use crate::input::Input;

pub enum Container {
    MainPanel(MainPanel),
}

impl Container {
    pub fn handle_input(&mut self, event: WindowEvent) {
        match self {
            Container::MainPanel(main_panel) => {
                main_panel.ui.handle_input(event);
            }
        }
    }
}

pub struct Window {
    pub content: Vec<Container>,
    pub input: Input,
}

impl Window {
    pub fn new() -> Window {
        let input = Input::new();
        Window {
            content: Vec::new(),
            input,
        }
        
    }
    pub fn handle_input(&mut self, event: WindowEvent) {
        self.input.handle_input(event);
    }
}

impl Draw for Window {
    fn draw(&self, app:&App, frame: &Frame) {
        self.content.iter().for_each(|container| {
            match container {   
                Container::MainPanel(main_panel) => {
                    main_panel.ui.draw(app, frame);
                }
            }
        });
    }
}

pub struct MainPanel {
    pub ui: Ui
}