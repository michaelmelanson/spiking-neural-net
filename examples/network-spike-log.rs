extern crate freude;
extern crate tuple;
extern crate rayon;
extern crate rand;
extern crate spiking_neural_net;

use std::time;
use std::thread::sleep;

use spiking_neural_net::network::Network;
use spiking_neural_net::neuron::models::HindmarshRoseNeuron;

fn main() {
    let dt = 0.01;
    let num_neurons = 2;
    let mut network = Network::<HindmarshRoseNeuron>::new(num_neurons, dt);

    let wall_clock = time::Instant::now();

    loop {
        let network_time_ms = (1000. * network.time) as u64;
        let network_time = time::Duration::from_millis(network_time_ms);
        let real_time = wall_clock.elapsed();
        if network_time > real_time {
            let diff = network_time - real_time;
            sleep(diff);
        }

        let results = network.advance();

        for result in results {
            if result.spike_start {
                println!("t={}: neuron {} is spiking", network_time_ms, result.id);
            }
        }
    }
}
