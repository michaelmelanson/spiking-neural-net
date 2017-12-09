extern crate freude;
extern crate tuple;
extern crate rayon;
extern crate rand;
extern crate spiking_neural_net;

use std::time;
use std::thread::sleep;

use spiking_neural_net::network::Network;
use spiking_neural_net::neuron::models::HindmarshRoseNeuron;

use rayon::prelude::*;

fn main() {
    let dt = 0.01;
    let mut network = Network::<HindmarshRoseNeuron>::new(1, dt);

    let wall_clock = time::Instant::now();

    while network.time < 10. {
        let network_time_ms = (1000. * network.time) as u64;
        let network_time = time::Duration::from_millis(network_time_ms);
        let real_time = wall_clock.elapsed();
        if network_time > real_time {
            let diff = network_time - real_time;
            sleep(diff);
        }

        network.advance();

        network.neurons.par_iter_mut().for_each(|n| {
            println!("{}, {}, {}, {}, {}",
                     network_time_ms,
                     n.state.0,
                     n.state.1,
                     n.state.2,
                     n.state.3);
        });
    }
}
