/// A synapse receives action potentials (spikes) from a presynaptic neuron and converts it to a
/// postsynaptic potential.

use specs::prelude::*;
use std::collections::BinaryHeap;
use crate::simulation::Time;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct PendingSpike(pub Time);

#[derive(Component)]
#[storage(VecStorage)]
pub struct Synapse {
    pub pre_neuron: Entity,
    pub post_neuron: Entity,
    pub delay: usize,
    pub strength: f64,
    pub pending_spikes: BinaryHeap<PendingSpike>
}
