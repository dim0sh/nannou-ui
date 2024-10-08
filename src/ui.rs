use nannou::{color, prelude::*};

pub trait Draw {
    fn draw(&self, app:&App, frame: &Frame);
}

pub struct Label {
    content: String,
    size: Vec2,
    pub position: Vec2,
    color: color::Rgb8,
}

impl Label {
    pub fn new_label(content: String, size: (u32,u32), position: (u32,u32), color: color::Rgb8) -> Label {
        Label {
            content,
            size: Vec2::new(size.0 as f32, size.1 as f32),
            position: Vec2::new(position.0 as f32, position.1 as f32),
            color,
        }
    }
    
    pub fn text(&mut self, content:String) {
        self.content = content;
    }
}

impl Draw for Label {
    fn draw(&self,app: &App,frame: &Frame) {
        let draw = app.draw();
        draw.rect()
            .wh(self.size)
            .xy(self.position)
            .color(self.color);
        draw.text(&self.content)
            .wh(self.size)
            .xy(self.position);
        draw.to_frame(app, &frame).unwrap();
    }
}

pub struct Button {
    content: String,
    size: Vec2,
    pub position: Vec2,
    color: color::Rgb8,
    pub is_pressed: bool,
}

impl Button {
    pub fn new_button(content: String, size: (u32,u32), position: (u32,u32), color: color::Rgb8) -> Button {
        Button {
            content,
            size: Vec2::new(size.0 as f32, size.1 as f32),
            position: Vec2::new(position.0 as f32, position.1 as f32),
            color,
            is_pressed: false,
        } 
    }
    
    pub fn update(&mut self, event: &WindowEvent,mouse_position: Vec2) {
        match event {
            MousePressed(mouse_button) => {
                if *mouse_button == MouseButton::Left && mouse_position.distance(self.position) < self.size.x{
                    self.is_pressed = true;
                }
            }
            MouseReleased(mouse_button) => {
                if *mouse_button == MouseButton::Left {
                    self.is_pressed = false;
                }
            }
            _ => {}
        }
    }

}

impl Draw for Button {
    fn draw(&self,app: &App,frame: &Frame) {
        let draw = app.draw();
        draw.rect()
            .wh(self.size)
            .xy(self.position)
            .color(self.color);
        draw.text(&self.content)
            .wh(self.size)
            .xy(self.position);
        draw.to_frame(app, &frame).unwrap();
    }
}

pub enum UiElem {
    Button(Button),
    Label(Label),
}

impl Draw for UiElem {
    fn draw(&self, app:&App, frame: &Frame) {
        match self {
            UiElem::Button(x) => x.draw(app,frame),
            UiElem::Label(x) => x.draw(app,frame),
        }
    }
}

pub struct Container {
    content: Vec<UiElem>
}

impl Container {
    pub fn new(content:Vec<UiElem>) -> Container {
        Container {
            content
        }
    }
}

impl Draw for Container {
    fn draw(&self, app:&App, frame: &Frame) {
        self.content.iter().for_each(|x| x.draw(app, frame));
    }
}