use nannou::{color, prelude::*};
use crate::{ui, ui::Draw, ui::UiElem};

pub struct Model {
    mouse_position: Vec2,
    // button: ui::Button,
    // label: ui::Label,
    // counter: ui::Label,
    count: u32,
    container: ui::Container,
}

pub fn model(app: &App) -> Model {
    app.new_window()
    .size(800,800)
    .view(view)
    .event(window_event)
    .build().unwrap();

    let label = ui::Label::new_label(String::from("HELLO"), (20,20), (0,0), RED);
    let counter = ui::Label::new_label(String::from(""), (20,20), (300,0), BLACK);
    let button = ui::Button::new_button(String::from("HELLO"), (20,20), (200,0), BLUE);

    let container = ui::Container::new(
        vec![
            UiElem::Label(label),
            UiElem::Label(counter),
            UiElem::Button(button),
        ]
    );

    Model { 
        mouse_position: Vec2::new(0.0, 0.0),
        // button,
        // label,
        // counter,
        count: 0,
        container,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    
    model.container.draw(app, &frame);

    // model.button.draw(app,&frame);
    // model.label.draw(app,&frame);
    // model.counter.draw(app,&frame);

    draw.to_frame(app, &frame).unwrap();
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    // if model.button.is_pressed {
    //     model.count += 1;
    //     model.counter.text(model.count.to_string());
    // }
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    // model.button.update(&event,model.mouse_position);
    match event {
        MouseMoved(mouse_pos) => {
            model.mouse_position = mouse_pos;
        }
        _ => {}
    }
}