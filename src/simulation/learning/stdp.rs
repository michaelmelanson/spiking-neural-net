use specs::prelude::*;
use specs_derive::Component;

use crate::simulation::components::synapse::Synapse;
use crate::simulation::components::neuron::ActionPotential;

#[derive(Component, Debug)]
pub struct STDPLearningRule {
    pre_dt: u64,
    post_dt: u64
}

impl Default for STDPLearningRule {
    fn default() -> Self {
        STDPLearningRule {
            pre_dt: std::u64::MAX,
            post_dt: std::u64::MAX
        }
    }
}

pub struct STDPLearningSystem;

impl <'a> System<'a> for STDPLearningSystem {
    type SystemData = (
        ReadStorage<'a, ActionPotential>,
        WriteStorage<'a, Synapse>,
        WriteStorage<'a, STDPLearningRule>
    );

    fn run(&mut self, (action_potentials, mut synapses, mut stdps): Self::SystemData) {

        (&mut synapses, &mut stdps).par_join().for_each(|(synapse, mut stdp)| {

            // the maximal change in synaptic strength
            let max_ltp = 0.1;
            let max_ltd = -0.1;

            let half_life = 20.; // total guess

            let mut stdp_effect = 0.;

            // did the presynaptic neuron spike? if so, apply long-term depression
            if action_potentials.get(synapse.pre_neuron).is_some() {
                stdp.pre_dt = 0;

                let dt = stdp.post_dt as f64;
                stdp_effect += max_ltd / (1. + (dt / half_life));
            }

            // did the postsynaptic neuron spike? if so, apply long-term potentiation
            if action_potentials.get(synapse.post_neuron).is_some() {
                stdp.post_dt = 0;

                let dt = stdp.pre_dt as f64;
                stdp_effect += max_ltp / (1. + (dt / half_life));
            }

            synapse.strength += stdp_effect;
        });
    }
}