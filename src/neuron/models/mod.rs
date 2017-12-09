use std;

pub trait NeuronModel<M: NeuronMorphology> {
    fn new(id: usize, num_neurons: usize, dt: f64, morphology: &M) -> Self;

    fn apply_epsps(&mut self, epsp_times: &Vec<f64>, time: f64);
    fn advance(&mut self, dt: f64);

    fn id(&self) -> usize;
    fn is_spiking(&self) -> bool;
}

pub trait NeuronMorphology
    where Self: std::marker::Sized
{
    type Model: NeuronModel<Self> + Send + Sync;
}

mod hindmarsh_rose;
pub use self::hindmarsh_rose::*;
