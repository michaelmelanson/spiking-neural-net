extern crate serde;
extern crate serde_yaml;
extern crate rayon;

extern crate spiking_neural_net;

#[macro_use]
extern crate clap;

use std::time;
use std::thread::sleep;

use clap::{Arg, App};

use std::fs::File;

use spiking_neural_net::network::Network;
use spiking_neural_net::neuron::models::HindmarshRoseMorphology;

use rayon::prelude::*;

fn main() {
    let matches = App::new("Single neuron trace")
        .author(crate_authors!())
        .about("Simulates a single neurons and prints a trace of its activity.")
        .arg(Arg::with_name("realtime")
            .long("real-time")
            .help("Maintain pace with the wall clock"))
        .arg(Arg::with_name("morphology")
            .long("morphology")
            .takes_value(true)
            .help("Path to a file containing a description of a neuron type")
            .required(true))
        .get_matches();

    let real_time: bool = matches.is_present("realtime");
    let dt = 0.01;

    let morphology_path = matches.value_of("morphology").expect("required parameter not provided");
    let morphology_file = File::open(morphology_path).expect("can't open morphology");
    let morphology: HindmarshRoseMorphology = serde_yaml::from_reader(morphology_file).unwrap();

    let mut network = Network::<HindmarshRoseMorphology>::new(1, dt, &morphology);

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
            } else if real_time.as_secs() > 0 && // this happens spuriously early in the run
                    network_time + slippage < real_time {
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
