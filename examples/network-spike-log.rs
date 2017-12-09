extern crate freude;
extern crate tuple;
extern crate rayon;
extern crate rand;
extern crate spiking_neural_net;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate clap;

use std::time;
use std::thread::sleep;
use std::fs::File;

use clap::{Arg, App};

use spiking_neural_net::network::Network;
use spiking_neural_net::neuron::models::HindmarshRoseMorphology;

fn main() {
    let matches = App::new("Network spike log")
        .author(crate_authors!())
        .about("Simulates a network of neurons and prints a log of when they spike.")
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .value_name("SIZE")
            .help("The number of neurons")
            .required(true)
            .takes_value(true))
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
    let num_neurons = value_t!(matches, "size", usize).unwrap();

    let morphology_path = matches.value_of("morphology").expect("required parameter not provided");
    let morphology_file = File::open(morphology_path).expect("can't open morphology");
    let morphology: HindmarshRoseMorphology = serde_yaml::from_reader(morphology_file).unwrap();

    let mut network = Network::<HindmarshRoseMorphology>::new(num_neurons, dt, &morphology);

    let wall_clock = time::Instant::now();

    let mut slippage = time::Duration::new(0, 0);

    loop {
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
                println!("Warning: Fallen behind real-time by {:?}", diff);
                slippage = diff;
            }
        }


        let results = network.advance();

        for result in results {
            if result.spike_start {
                println!("t={}: neuron {} is spiking", network_time_ms, result.id);
            }
        }
    }
}
