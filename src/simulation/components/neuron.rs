use specs::prelude::*;

#[derive(Component, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct Neuron {
    pub psp: f64 // postsynaptic potential
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Spiking;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct ActionPotential;
