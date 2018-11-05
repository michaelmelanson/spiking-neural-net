use specs::prelude::*;
use rand::distributions::{
    Normal,
    Distribution
};

pub mod components;
pub mod models;

use self::components::neuron::*;
use self::components::synapse::*;

use simulation::models::hindmarsh_rose::{
    HindmarshRoseModel,
    HindmarshRoseMorphology,
    HindmarshRoseIntegrator,
    HindmarshRoseCSVWriter
};


#[derive(Default)]
pub struct SimulationTime(pub u64);


pub fn run() {
    let mut world = World::new();
    world.register::<Neuron>();
    world.register::<Spiking>();

    world.register::<HindmarshRoseModel>();
    world.register::<HindmarshRoseMorphology>();

    let normal = Normal::new(0.0, 3.0);

    // create a bunch of neurons
    {
        let num_neurons: u32 = 2;
        for _ in 0..num_neurons {
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
    }

    // wire them up with synapses
    {
        let entities = world.entities();
        let neurons = world.read_storage::<Neuron>();
        let mut synapses = world.write_storage::<Synapse>();
        for (pre_neuron, neuron) in (&entities, &neurons).join() {
            for (post_neuron, neuron) in (&entities, &neurons).join() {
                if pre_neuron == post_neuron {
                    break;
                }

                entities.build_entity()
                    .with(Synapse { pre_neuron, post_neuron }, &mut synapses)
                    .build();
            }
        }
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
