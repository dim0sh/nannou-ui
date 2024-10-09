use nannou;
mod app;
mod ui;
mod input;
mod ui_elem;

use app::model;
use app::update;

fn main() {
    nannou::app(model).update(update).run();
}
