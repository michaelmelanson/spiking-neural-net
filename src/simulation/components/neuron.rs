use specs::prelude::*;
use std::slice::Iter;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Neuron {
    pub psp: f64 // postsynaptic potential
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Spiking;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct ActionPotential;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Sensory,
    Motor,
    Afferent,
    Efferent,
    Internal
}

impl Layer {
    pub fn iter() -> Iter<'static, Layer> {
        static LAYERS: [Layer;  5] = [
            Layer::Sensory,
            Layer::Motor,
            Layer::Afferent,
            Layer::Efferent,
            Layer::Internal
        ];

        LAYERS.iter()
    }
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ColumnCoordinates {
    pub column: usize,
    pub layer: Layer
}