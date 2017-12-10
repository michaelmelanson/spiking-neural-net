use std;

pub trait NeuronModel<M: NeuronMorphology> {
    fn new(id: usize, num_neurons: usize, dt: f64, morphology: &M) -> Self;

    fn apply_epsps(&mut self, epsp_times: &Vec<f64>, time: f64);
    fn step(&mut self, epsp_times: &Vec<f64>, time: f64, dt: f64) -> TimeStepResult;

    fn id(&self) -> usize;
    fn is_spiking(&self) -> bool;
}

pub trait NeuronMorphology
    where Self: std::marker::Sized
{
    type Model: NeuronModel<Self> + Send + Sync;
}

#[derive(Debug)]
pub struct TimeStepResult {
    pub id: usize,
    pub spike_start: bool,
    pub spike_end: bool,
}


mod hindmarsh_rose;
pub use self::hindmarsh_rose::*;
