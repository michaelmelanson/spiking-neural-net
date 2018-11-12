
#[macro_use] extern crate specs_derive;
extern crate specs;
extern crate rayon;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate rand;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate core;

mod simulation;

fn main() {
    pretty_env_logger::init();
    simulation::run();
}
