use nannou;
mod app;
mod ui;

use app::model;
use app::update;

fn main() {
    nannou::app(model).update(update).run();
}
