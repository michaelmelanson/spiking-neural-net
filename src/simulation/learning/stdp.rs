use specs::prelude::*;

use simulation::components::synapse::Synapse;
use simulation::components::neuron::ActionPotential;

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
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

        (&mut synapses, &mut stdps).par_join().for_each(|(synapse, stdp)| {
            let max_ltp = 0.1;
            let max_ltd = -0.1;

            let half_life = 20.;

            let mut stdp_effect = 0.;

            if action_potentials.get(synapse.pre_neuron).is_some() {
                stdp.pre_dt = 0;

                // apply long-term depression (LTD)
                let dt = stdp.post_dt as f64;
                stdp_effect += max_ltd / (1. + (dt / half_life));
            }

            if action_potentials.get(synapse.post_neuron).is_some() {
                stdp.post_dt = 0;

                // apply long-term potentiation (LTP)
                let dt = stdp.pre_dt as f64;
                stdp_effect += max_ltp /
                    (1. + (dt / half_life));
            }

            synapse.strength += stdp_effect;
        });
    }
}