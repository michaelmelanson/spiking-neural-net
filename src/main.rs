
#[macro_use] extern crate specs_derive;
extern crate specs;
extern crate rayon;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate rand;

mod simulation;
mod hindmarsh_rose;

fn main() {
    pretty_env_logger::init();
    simulation::run();
}
