use nannou::{color, prelude::*};
use crate::ui::{Ui, Draw};
use crate::input::Input;
use crate::ui_elem::{Button, UiElem};

pub struct Model {
    ui: Ui,
    count: u32,
}

pub fn model(app: &App) -> Model {
    app.new_window()
    .size(800,800)
    .view(view)
    .event(window_event)
    .build().unwrap();
    
    let mut ui = Ui::new();

    Model {
        ui,
        count: 0,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.ui.draw(app, &frame);

    draw.to_frame(app, &frame).unwrap();
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    model.ui.refresh();
    model.ui.add(
        UiElem::Button(
            Button::new(String::from("test") ,(40,40), (0,0), RED)
        )
    );
    if model.ui.add(
        UiElem::Button(
            Button::new(model.count.to_string(), (40,40), (100,100), BLUE)
        )
    ).clicked() {
        model.count += 1;
    };
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    model.ui.handle_input(event);
}