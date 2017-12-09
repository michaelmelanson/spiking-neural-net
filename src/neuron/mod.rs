pub mod models;

pub use self::models::*;

#[derive(Debug)]
pub struct TimeStepResult {
    pub id: usize,
    pub spike_start: bool,
    pub spike_end: bool,
}

pub fn advance_neuron<N: NeuronModel<M>, M: NeuronMorphology>(neuron: &mut N,
                                                              epsp_times: &Vec<f64>,
                                                              time: f64,
                                                              dt: f64)
                                                              -> TimeStepResult {
    let was_spiking = neuron.is_spiking();

    neuron.apply_epsps(epsp_times, time);
    neuron.advance(dt);

    let is_spiking = neuron.is_spiking();

    TimeStepResult {
        id: neuron.id(),
        spike_start: is_spiking && !was_spiking,
        spike_end: was_spiking && !is_spiking,
    }
}
