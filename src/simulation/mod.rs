use specs::prelude::*;
use log::info;

use rand::distributions::{
    Distribution,
    Uniform,
    Bernoulli
};

pub mod components;
pub mod models;
pub mod learning;
pub mod systems;

use self::components::neuron::*;
use self::components::synapse::*;
use self::learning::stdp::*;

use crate::{
    simulation::models::{
        izhikevich::{
            IzhikevichModel,
            IzhikevichMorphology,
            IzhikevichIntegrator
        },
        hindmarsh_rose::{
            HindmarshRoseModel,
            HindmarshRoseMorphology,
            HindmarshRoseIntegrator
        },
    },
    simulation::systems::{
        synaptic_transmission::SynapticTransmissionSystem,
        csv_writer::CSVWriterSystem
    },
};

use std::collections::BinaryHeap;


pub type Time = usize;
#[derive(Default)]
pub struct SimulationTime(pub Time);


pub fn run() {
    info!("Setting up simulation...");

    let mut world = World::new();
    world.register::<Neuron>();
    world.register::<ColumnCoordinates>();
    world.register::<Spiking>();
    world.register::<ActionPotential>();
    world.register::<HindmarshRoseModel>();
    world.register::<HindmarshRoseMorphology>();
    world.register::<IzhikevichModel>();
    world.register::<IzhikevichMorphology>();
    world.register::<Synapse>();
    world.register::<STDPLearningRule>();

    world.add_resource(SimulationTime::default());

    info!("Generating network...");
    let synaptic_delay = Uniform::new(1, 20);
    let synaptic_strength = Uniform::new(0.5, 4.0);
    let excitory = Bernoulli::new(0.8); // what fraction of synapses are excitory

    let regular_spiking = IzhikevichMorphology {
        a: 0.02,
        b: 0.2,
        c: -65.,
        d: 2.
    };

    info!("  - Neurons");

    // create a bunch of neurons
    {
        let mut neurons_created = 0;
        let num_columns = 20;

        for column in 0..num_columns {
            for layer in Layer::iter() {
                for _ in 0..match layer {
                    Layer::Motor    => 10,
                    Layer::Sensory  => 10,
                    Layer::Afferent => 10,
                    Layer::Efferent => 10,
                    Layer::Internal => 60
                } {
                    world.create_entity()
                        .with(Neuron::default())
                        .with(ColumnCoordinates {
                            column,
                            layer: *layer
                        })
                        .with(IzhikevichModel { v: -65., u: 0.02 * -65. })
                        .with(regular_spiking.clone())
                        .build();

                    neurons_created += 1;
                }
            }
        }

        info!("Created {} neurons", neurons_created);
    }

    info!("  - Synapses");

    // wire them up with synapses
    {
        let mut synapses_created = 0;
        let entities = world.entities();
        let neurons = world.read_storage::<Neuron>();
        let coordinates = world.read_storage::<ColumnCoordinates>();
        let mut synapses = world.write_storage::<Synapse>();
        let mut stdps = world.write_storage::<STDPLearningRule>();

        let mut rng = rand::thread_rng();

        for (pre_neuron, _, pre_coord) in (&entities, &neurons, &coordinates).join() {
            for (post_neuron, _, post_coord) in (&entities, &neurons, &coordinates).join() {
                let probability = match (pre_coord.column == post_coord.column, pre_coord.layer, post_coord.layer) {

                    // intra-column, intra-layer connections
                    (true, Layer::Internal, Layer::Internal)  => 0.8,
                    (true, x, y) if x == y                    => 0.4,

                    // intra-column, cross-layer connections
                    (true, Layer::Sensory,   Layer::Internal) => 0.8,
                    (true, Layer::Afferent,  Layer::Internal) => 0.8,
                    (true, Layer::Internal,  Layer::Motor)    => 0.8,
                    (true, Layer::Internal,  Layer::Efferent) => 0.8,

                    // cross-column connections
                    (false, Layer::Efferent, Layer::Afferent) => 0.3,

                    // everything else is not connected
                    (_, _, _) => 0.
                };

                if probability > 0. {
                    let has_synapse = Bernoulli::new(probability);
                    if has_synapse.sample(&mut rng) {
                        let delay = synaptic_delay.sample(&mut rng);
                        let is_excitory = excitory.sample(&mut rng);
                        let strength = match is_excitory {
                            true => synaptic_strength.sample(&mut rng),
                            false => -synaptic_strength.sample(&mut rng)
                        };

                        entities.build_entity()
                            .with(Synapse {
                                pre_neuron,
                                post_neuron,
                                delay,
                                strength,
                                pending_spikes: BinaryHeap::new()
                            }, &mut synapses)
                            .with(STDPLearningRule::default(), &mut stdps)
                            .build();
                        synapses_created += 1;
                    }
                }
            }
        }

        info!("Created {} synapses", synapses_created);
    }

    info!("Starting simulation...");

    let mut dispatcher = DispatcherBuilder::new()
        .with(HindmarshRoseIntegrator, "hindmarsh_rose_integrator", &[])
        .with(IzhikevichIntegrator, "izhikevich_integrator", &[])

        .with(SynapticTransmissionSystem, "synaptic_transmission", &["hindmarsh_rose_integrator", "izhikevich_integrator"])
        .with(STDPLearningSystem, "stdp_learning", &["synaptic_transmission"])

        .with(CSVWriterSystem::new(), "csv_writer", &["synaptic_transmission"])

        .build();


    loop {
        {
            let mut time = world.write_resource::<SimulationTime>();
            time.0 += 1;
            if time.0 > 20000 {
                break;
            }

            if time.0 % 1000 == 0 {
                info!("Time {}", time.0);
            }

        }

        dispatcher.dispatch(&mut world);
        world.maintain();
    }
}
