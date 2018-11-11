use specs::prelude::*;
use rand::distributions::{
    Distribution,
    Uniform,
    Bernoulli
};

pub mod components;
pub mod models;
pub mod systems;

use self::components::neuron::*;
use self::components::synapse::*;

use simulation::models::hindmarsh_rose::{
    HindmarshRoseModel,
    HindmarshRoseMorphology,
    HindmarshRoseIntegrator
};

use simulation::models::izhikevich::{
    IzhikevichModel,
    IzhikevichMorphology,
    IzhikevichIntegrator
};

use simulation::systems::synaptic_transmission::SynapticTransmissionSystem;
use simulation::systems::csv_writer::CSVWriterSystem;


#[derive(Default)]
pub struct SimulationTime(pub u64);


pub fn run() {
    let mut world = World::new();
    world.register::<Neuron>();
    world.register::<Spiking>();
    world.register::<Synapse>();
    world.register::<ActionPotential>();
    world.register::<DelayedPotential>();

    world.register::<HindmarshRoseModel>();
    world.register::<HindmarshRoseMorphology>();

    world.register::<IzhikevichModel>();
    world.register::<IzhikevichMorphology>();

    let has_synapse = Bernoulli::new(0.8);
    let synaptic_delay = Uniform::new(1, 20);
    let synaptic_strength = Uniform::new(0.5, 1.0);
    let excitory = Bernoulli::new(0.8); // what fraction of synapses are excitory

    let regular_spiking = IzhikevichMorphology {
        a: 0.02,
        b: 0.2,
        c: -65.,
        d: 2.
    };

    // create a bunch of neurons
    {
        let num_neurons: u32 = 5;
        for _ in 0..num_neurons {
            world.create_entity()
                .with(Neuron::default())
                .with(IzhikevichModel { v: -65., u: 0.02 * -65. })
                .with(regular_spiking.clone())

                /*
                .with(HindmarshRoseModel {
                    x: model_param.sample(&mut rand::thread_rng()),
                    y: model_param.sample(&mut rand::thread_rng()),
                    z: model_param.sample(&mut rand::thread_rng()),
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
                */
                .build();
        }
    }

    // wire them up with synapses
    {
        let entities = world.entities();
        let neurons = world.read_storage::<Neuron>();
        let mut synapses = world.write_storage::<Synapse>();

        let mut rng = rand::thread_rng();

        for (pre_neuron, _) in (&entities, &neurons).join() {
            for (post_neuron, _) in (&entities, &neurons).join() {
                if has_synapse.sample(&mut rng) {
                    let delay = synaptic_delay.sample(&mut rng);
                    let is_excitory = excitory.sample(&mut rng);
                    let strength = match is_excitory {
                        true => synaptic_strength.sample(&mut rng),
                        false => -synaptic_strength.sample(&mut rng)
                    };

                    entities.build_entity()
                        .with(Synapse { pre_neuron, post_neuron, delay, strength }, &mut synapses)
                        .build();
                }
            }
        }
    }

    world.add_resource(SimulationTime::default());

    let mut dispatcher = DispatcherBuilder::new()
        .with(HindmarshRoseIntegrator, "hindmarsh_rose_integrator", &[])
        .with(IzhikevichIntegrator, "izhikevich_integrator", &[])

        .with(CSVWriterSystem, "csv_writer", &["hindmarsh_rose_integrator", "izhikevich_integrator"])

        .with(SynapticTransmissionSystem, "synaptic_transmission", &["hindmarsh_rose_integrator", "izhikevich_integrator"])
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
