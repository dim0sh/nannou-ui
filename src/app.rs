use nannou::prelude::*;
use crate::ui::{Ui, Draw};
use crate::ui_elem::{Button, Label, UiElem};

pub struct Model {
    ui: Ui,
    count: u32,
}

pub fn model(app: &App) -> Model {
    let size: (u32,u32) = (800,800);
    app.new_window()
    .size(size.0,size.1)
    .view(view)
    .event(window_event)
    .build().unwrap();
    
    let ui = Ui::new(size);

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

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.ui.refresh();
    model.ui.add(
        UiElem::Label(
            Label::new("test".to_string() ,(40,40), (0,0), RED)
        )
    );
    if model.ui.add(
        UiElem::Button(
            Button::new(model.count.to_string(), (100,40), (0,100), BLUE)
        )
    ).clicked() {
        model.count += 1;
    };
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    model.ui.handle_input(event);
}