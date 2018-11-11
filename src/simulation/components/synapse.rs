/// A synapse receives action potentials (spikes) from a presynaptic neuron and converts it to a
/// postsynaptic potential.

use specs::prelude::*;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Synapse {
    pub pre_neuron: Entity,
    pub post_neuron: Entity,
    pub delay: usize,
    pub strength: f64
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct DelayedPotential {
    pub post_neuron: Entity,
    pub time_to_psp: usize,
    pub psp_amp: f64
}
