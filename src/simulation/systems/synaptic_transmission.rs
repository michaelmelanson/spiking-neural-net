use specs::prelude::*;

use simulation::components::neuron::*;
use simulation::components::synapse::*;

pub struct SynapticTransmissionSystem;

impl <'a> System<'a> for SynapticTransmissionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Synapse>,

        WriteStorage<'a, Neuron>,
        WriteStorage<'a, ActionPotential>,
        WriteStorage<'a, DelayedPotential>
    );

    fn run(&mut self, (entities, updater, synapses, mut neurons, action_potentials, mut delayed_potentials): Self::SystemData) {

        // turn action potentials into delayed spikes
        // also clear the PSP
        (&synapses).par_join().for_each(|synapse| {
            if action_potentials.get(synapse.pre_neuron).is_some() {
                updater.create_entity(&entities)
                    .with(DelayedPotential {
                        post_neuron: synapse.post_neuron,
                        time_to_psp: synapse.delay,
                        psp_amp: synapse.strength
                    })
                    .build();
            }
        });


        // clear all the action potentials because they're now processed
        (&entities, &action_potentials).par_join().for_each(|(entity, _action_potential)| {
            updater.remove::<ActionPotential>(entity);
        });

        // update any delayed potentials
        // if any are now due, turn them into post-synaptic potentials
        for (entity, mut delayed_potential) in (&entities, &mut delayed_potentials).join() {
            delayed_potential.time_to_psp -= 1;

            if delayed_potential.time_to_psp == 0 {
                let post = neurons.get_mut(delayed_potential.post_neuron).unwrap();
                post.psp += delayed_potential.psp_amp;
                updater.remove::<DelayedPotential>(entity);
            }
        }
    }
}