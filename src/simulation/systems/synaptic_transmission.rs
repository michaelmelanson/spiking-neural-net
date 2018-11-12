use specs::prelude::*;

use simulation::components::neuron::*;
use simulation::components::synapse::*;
use simulation::SimulationTime;

pub struct SynapticTransmissionSystem;

impl <'a> System<'a> for SynapticTransmissionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, SimulationTime>,
        WriteStorage<'a, Synapse>,
        WriteStorage<'a, ActionPotential>
    );

    fn run(&mut self, (entities, updater, simulation_time, mut synapses, action_potentials): Self::SystemData) {
        let time = (*simulation_time).0;

        // turn action potentials into delayed spikes
        (&mut synapses).par_join().for_each(|synapse| {
            if action_potentials.get(synapse.pre_neuron).is_some() {
                let spike_time = time + synapse.delay;
                synapse.pending_spikes.push(PendingSpike(spike_time));
            }
        });


        // clear all the action potentials because they're now processed
        (&entities, &action_potentials).par_join().for_each(|(entity, _action_potential)| {
            updater.remove::<ActionPotential>(entity);
        });

        // update any delayed potentials
        // if any are now due, turn them into post-synaptic potentials
        (&mut synapses).par_join().for_each(|synapse| {
            if !synapse.pending_spikes.is_empty() && synapse.pending_spikes.peek().unwrap().0 <= time {
                synapse.pending_spikes.pop();

                let post_neuron = synapse.post_neuron;
                let psp_amp = synapse.strength;

                updater.exec_mut(move |world| {
                    let mut neurons = world.write_storage::<Neuron>();
                    let post = neurons.get_mut(post_neuron).unwrap();
                    post.psp += psp_amp;

                    debug!("Neuron {} got spike at t={} with amplitude {} (current is now {}).",
                           post_neuron.id(), time, psp_amp, post.psp);

                });
            }
        });
    }
}