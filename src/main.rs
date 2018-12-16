#[macro_use] extern crate specs_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

mod simulation;

fn main() {
    pretty_env_logger::init();
    simulation::run();
}
