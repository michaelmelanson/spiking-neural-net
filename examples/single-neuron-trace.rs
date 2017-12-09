extern crate freude;
extern crate tuple;
extern crate rayon;
extern crate rand;
extern crate spiking_neural_net;

#[macro_use]
extern crate clap;

use std::time;
use std::thread::sleep;

use clap::{Arg, App};

use spiking_neural_net::network::Network;
use spiking_neural_net::neuron::models::HindmarshRoseNeuron;

use rayon::prelude::*;

fn main() {
    let matches = App::new("Single neuron trace")
        .author(crate_authors!())
        .about("Simulates a single neurons and prints a trace of its activity.")
        .arg(Arg::with_name("realtime")
            .long("real-time")
            .help("Maintain pace with the wall clock"))
        .get_matches();

    let real_time: bool = matches.is_present("realtime");
    let dt = 0.01;
    let mut network = Network::<HindmarshRoseNeuron>::new(1, dt);

    let wall_clock = time::Instant::now();

    let mut slippage = time::Duration::new(0, 0);

    while network.time < 10. {
        let network_time_ms = (1000. * network.time) as u64;
        if real_time {
            let network_time = time::Duration::from_millis(network_time_ms);
            let real_time = wall_clock.elapsed();
            if network_time > real_time {
                let diff = network_time - real_time;
                sleep(diff);
            } else if network_time + slippage < real_time {
                let diff = real_time - network_time;
                println!("Fallen behind real-time by {:?}", diff);
                slippage = diff;
            }            
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
