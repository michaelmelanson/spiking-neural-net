use specs::prelude::*;
use rand::distributions::{
    Normal,
    Distribution
};

use hindmarsh_rose::{
    HindmarshRoseModel,
    HindmarshRoseMorphology,
    HindmarshRoseIntegrator,
    HindmarshRoseCSVWriter
};

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Neuron;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Spiking;

#[derive(Default)]
pub struct SimulationTime(pub u64);


pub fn run() {
    let mut world = World::new();
    world.register::<Neuron>();
    world.register::<Spiking>();

    world.register::<HindmarshRoseModel>();
    world.register::<HindmarshRoseMorphology>();

    let normal = Normal::new(0.0, 3.0);

    for _ in 0..10000 {
        world.create_entity()
            .with(Neuron)
            .with(HindmarshRoseModel {
                x: normal.sample(&mut rand::thread_rng()),
                y: normal.sample(&mut rand::thread_rng()),
                z: normal.sample(&mut rand::thread_rng()),
                i: 1.0
            })
            .with(HindmarshRoseMorphology {
                a: 1.0,
                b: 3.0,
                c: 1.0,
                d: 5.0,
                beta: 1.0,
                r: 0.001,
                s: 4.0,
                x_r: -1.6,
                t_s: 0.1,
                epsp_amp: 0.1
            })
            .build();
    }

    world.add_resource(SimulationTime::default());

    let mut dispatcher = DispatcherBuilder::new()
        .with(HindmarshRoseIntegrator, "hindmarsh_rose_integrator", &[])
        //.with(HindmarshRoseCSVWriter, "hindmarsh_rose_csv_writer", &["hindmarsh_rose_integrator"])
        .build();

    debug!("Starting simulation...");

    loop {
        {
            let mut time = world.write_resource::<SimulationTime>();
            time.0 += 1;
            if time.0 > 20000 {
                break;
            }

            info!("Time {}", time.0);
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
