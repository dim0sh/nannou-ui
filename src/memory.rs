use nannou::prelude::Vec2;

use crate::ui_elem::{self, UiElem};

pub struct Memory {
    content: Vec<UiElem>,
    app_size: Vec2,
}

impl Memory {
    pub fn new(app_size: Vec2) -> Memory {
        Memory {
            content: Vec::new(),
            app_size,
        }
    }
    pub fn update(&mut self,ui_elem: UiElem) {
        self.content.push(ui_elem);
    }
    pub fn clear(&mut self) {
        self.content.clear();
    } 
    pub fn get_ui(&self) -> &Vec<UiElem> {
        &self.content
    }
}