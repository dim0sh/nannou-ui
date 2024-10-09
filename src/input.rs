use nannou::{prelude::*};

#[derive(PartialEq)]
pub enum UiEvent {
    Default,
    MousePressedLeft,
    MouseReleasedLeft,
}

pub struct Input {
    pub pointer: Vec2,
    pub event: UiEvent,
}

impl Input {
    pub fn new() -> Input {
        Input {
            pointer: Vec2::new(0.0, 0.0),
            event: UiEvent::Default,
        }
    }

    pub fn handle_input(&mut self, event: WindowEvent) {
        match event {
            MouseMoved(mouse_pos) => {
                self.pointer = mouse_pos;
            }
            MousePressed(button) => {
                match button {
                    MouseButton::Left => {
                        self.event = UiEvent::MousePressedLeft
                    }
                    MouseButton::Right => {

                    }
                    _ => {}
                }
            }
            MouseReleased(button) => {
                match button {
                    MouseButton::Left => {
                        self.event = UiEvent::MouseReleasedLeft
                    }
                    MouseButton::Right => {

                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}