use controller::Controller;

mod controller;
mod view;

mod lib;
fn main() {
    let mut c = Controller::new();

    c.execute();
}