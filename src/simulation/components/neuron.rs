use specs::prelude::*;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Neuron;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Spiking;
