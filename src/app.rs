use nannou::prelude::*;
use crate::container;
use crate::ui::{Ui, Draw};
use crate::ui_elem::{Button, Label, UiElem};

pub struct Model {
    window: container::Window,
    // ui: Ui,
    count: u32,
    invert: bool,
}

pub fn model(app: &App) -> Model {
    let size: (u32,u32) = (800,800);
    app.new_window()
    .size(size.0,size.1)
    .view(view)
    .event(window_event)
    .build().unwrap();
    
    // let ui = Ui::new(size);

    Model {
        window: container::Window::new(),
        // ui,
        count: 0,
        invert: false,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // model.ui.draw(app, &frame);
    model.window.draw(app, &frame);

    draw.to_frame(app, &frame).unwrap();
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.window.content.clear();
    model.window.content.push(container::Container::MainPanel(container::MainPanel{
        ui: {
            let mut ui = Ui::new((800, 800),model.window.input.clone());
            ui.add(UiElem::Label(
                        Label::new("test".to_string() ,(40,40), (0,0), RED)
            ));
            if ui.add(
                UiElem::Button(
                    Button::new(model.count.to_string(), (100,40), (0,100), BLUE)
                )
                ).clicked() {
                    model.count += 1;
                };
            ui
        }
    }));
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    // model.ui.handle_input(event);
    model.window.handle_input(event);
}