use std::time::Duration;

use nannou::{color, glam::Vec2, time::{self, DurationF64}, App, Frame};
use crate::{input::{Input, UiEvent}, ui::Draw};


/// enum to handle all ui elements
#[derive(Clone)]
pub enum UiElem {
    // Container(Container),
    Button(Button),
    Label(Label),
}

impl UiElem {
    /// checks if is ui Element clicked
    pub fn clicked(&self) -> bool {
        match self {
            UiElem::Button(button) => {                
                button.is_pressed
            },
            UiElem::Label(_) => false,
        }
    }
}

/// Button ui element
#[derive(Clone)]
pub struct Button {
    text: String,
    pub is_pressed: bool,
    size: Vec2,
    position: Vec2,
    color: color::Rgb8,
}

impl Button {
    /// new Button
    pub fn new(text: String, size: (u32,u32), position: (u32,u32), color: color::Rgb8) -> Button {
        Button { 
            text,
            is_pressed: false, 
            size: Vec2::new(size.0 as f32, size.1 as f32), 
            position: Vec2::new(position.0 as f32, position.1 as f32),
            color, 
            // f 
        }
    }
    /// update Button is_pressed with event using Input
    pub fn update(&mut self, input: &Input) {
        if (input.pointer.x - self.position.x).abs() <= self.size.x/2.
            && (input.pointer.y - self.position.y).abs() <= self.size.y/2. 
            && input.event == UiEvent::MousePressedLeft {
                self.is_pressed = true;
        } else {
            self.is_pressed = false;
        }
    }

}
/// Draw trait for Button
impl Draw for Button {
    fn draw(&self, app:&App, frame: &Frame) {
        let draw = app.draw();
        draw.rect()
            .wh(self.size)
            .xy(self.position)
            .color(self.color);
        draw.text(&self.text)
            .wh(self.size)
            .xy(self.position);
        draw.to_frame(app, &frame).unwrap();
    }
}

/// Label ui element 
#[derive(Clone)]
pub struct Label {
    text: String,
    size: Vec2,
    position: Vec2,
    color: color::Rgb8,
}

impl Label {
    /// new Label
    pub fn new(text: String, size: (u32,u32), position: (u32,u32), color: color::Rgb8) -> Label {
        Label { 
            text,
            size: Vec2::new(size.0 as f32, size.1 as f32), 
            position: Vec2::new(position.0 as f32, position.1 as f32),
            color
        }
    }
}

/// draw trait for Label
impl Draw for Label {
    fn draw(&self, app:&App, frame: &Frame) {
        let draw = app.draw();
        draw.rect()
            .wh(self.size)
            .xy(self.position)
            .color(self.color);
        draw.text(&self.text)
            .wh(self.size)
            .xy(self.position);
        draw.to_frame(app, &frame).unwrap();
    }
}