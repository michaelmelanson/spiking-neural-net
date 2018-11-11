use specs::prelude::*;
use simulation::SimulationTime;
use simulation::models::izhikevich::IzhikevichModel;
use simulation::models::hindmarsh_rose::HindmarshRoseModel;

pub struct CSVWriterSystem;

impl <'a> System<'a> for CSVWriterSystem {
    type SystemData = (
        Read<'a, SimulationTime>,
        ReadStorage<'a, IzhikevichModel>,
        ReadStorage<'a, HindmarshRoseModel>,
    );

    fn run(&mut self, (time, izhikevich, hindmarsh_rose): Self::SystemData) {
        if time.0 == 1 {
            println!("time,");
        }

        print!("{}", time.0);

        for model in (&izhikevich).join() {
            print!(", {}", model.v);
        }

        for model in (&hindmarsh_rose).join() {
            print!(", {}", model.y);
        }

        println!();
    }
}