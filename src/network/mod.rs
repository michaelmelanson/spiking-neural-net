use std;
use rayon::prelude::*;

use super::neuron::{NeuronModel, NeuronMorphology, advance_neuron, TimeStepResult};

pub struct Network<M: NeuronMorphology> {
    pub neurons: Vec<M::Model>,
    epsp_times: Vec<f64>,
    dt: f64,
    pub time: f64,
}

impl<M: NeuronMorphology> Network<M> {
    pub fn new(num_neurons: usize, dt: f64, morphology: &M) -> Network<M> {
        let mut neurons: Vec<M::Model> = Vec::new();

        for i in 0..num_neurons {
            let neuron = M::Model::new(i as usize, num_neurons, dt, morphology);

            neurons.push(neuron);
        }

        let mut epsp_times = Vec::new();
        epsp_times.resize(neurons.len(), std::f64::MIN);

        Network {
            neurons: neurons,
            epsp_times: epsp_times,
            dt: dt,
            time: 0.,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn advance(&mut self) -> Vec<TimeStepResult> {
        let epsp_times = &mut self.epsp_times;
        let time = self.time;
        let dt = self.dt;
        let results = self.neurons
            .par_iter_mut()
            .map(|n| advance_neuron(n, &epsp_times, time, dt))
            .collect::<Vec<_>>();

        results.par_iter()
            .zip(epsp_times.par_iter_mut())
            .for_each(|(result, epsp_time)| if result.spike_start {
                *epsp_time = time;
            });

        self.time += self.dt;

        results
    }
}
