use nannou::{glam::Vec2, color, App, Frame};
use crate::{input::{Input, UiEvent}, ui::Draw};

#[derive(Clone)]
pub enum UiElem {
    // Container(Container),
    Button(Button),
    Label(Label),
}

impl UiElem {
    pub fn clicked(&self) -> bool {
        match self {
            UiElem::Button(button) => button.is_pressed,
            UiElem::Label(_) => false,
        }
    }
}

#[derive(Clone)]
pub struct Button {
    text: String,
    pub is_pressed: bool,
    size: Vec2,
    position: Vec2,
    color: color::Rgb8,
    // f: fn()
}

impl Button {
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
    pub fn update(&mut self, input: &Input) {

        if input.pointer.distance(self.position) < self.size.x 
            && input.event == UiEvent::MousePressedLeft {
                self.is_pressed = true;
        } else {
            self.is_pressed = false;
        }

        // if self.is_pressed {
        //     (self.f)()
        // }
    }

}

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
#[derive(Clone)]
pub struct Label {
    text: String,
    size: Vec2,
    position: Vec2,
    color: color::Rgb8,
}

impl Label {
    pub fn new(text: String, size: (u32,u32), position: (u32,u32), color: color::Rgb8) -> Label {
        Label { 
            text,
            size: Vec2::new(size.0 as f32, size.1 as f32), 
            position: Vec2::new(position.0 as f32, position.1 as f32),
            color
        }
    }
}

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