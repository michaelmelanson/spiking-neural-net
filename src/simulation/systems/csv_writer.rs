use specs::prelude::*;
use simulation::SimulationTime;
use simulation::models::izhikevich::IzhikevichModel;
use simulation::models::hindmarsh_rose::HindmarshRoseModel;
use std::fs::File;
use std::io::Write;
use simulation::components::neuron::ActionPotential;
use std::io::BufWriter;

pub struct CSVWriterSystem {
    pub trace_file: Option<BufWriter<File>>,
    pub spike_file: Option<BufWriter<File>>
}

impl CSVWriterSystem {
    pub fn new() -> Self {
        CSVWriterSystem {
            trace_file: Some(BufWriter::new(File::create("neuron-trace.csv").unwrap())),
            spike_file: Some(BufWriter::new(File::create("spikes.out").unwrap()))
        }
    }
}

impl <'a> System<'a> for CSVWriterSystem {
    type SystemData = (
        Read<'a, SimulationTime>,
        Entities<'a>,
        ReadStorage<'a, IzhikevichModel>,
        ReadStorage<'a, HindmarshRoseModel>,
        ReadStorage<'a, ActionPotential>
    );

    fn run(&mut self, (time, entities, izhikevich, hindmarsh_rose, action_potentials): Self::SystemData) {
        let trace_file = &mut self.trace_file;
        let spike_file = &mut self.spike_file;

        if time.0 == 1 {
            if let Some(ref mut trace_file) = trace_file {
                write!(trace_file, "time");

                let mut neuron_id = 0;

                for _model in (&izhikevich).join() {
                    neuron_id += 1;
                    write!(trace_file, ", neuron {} membrane potential", neuron_id);
                }

                for _model in (&hindmarsh_rose).join() {
                    neuron_id += 1;
                    write!(trace_file, ", neuron {} membrane potential", neuron_id);
                }

                writeln!(trace_file);
            }
        }

        if let Some(ref mut trace_file) = trace_file {
            write!(trace_file, "{}", time.0);
        }

        for (entity, model) in (&entities, &izhikevich).join() {
            if let Some(ref mut trace_file) = trace_file {
                write!(trace_file, ", {}", model.v);
            }

            if let Some(ref mut spike_file) = spike_file {
                write!(spike_file, "{} ", match action_potentials.get(entity) {
                    None => 0,
                    Some(_) => 1
                });
            }
        }

        for (entity, model) in (&entities, &hindmarsh_rose).join() {
            if let Some(ref mut trace_file) = trace_file {
                write!(trace_file, ", {}", model.y);
            }

            if let Some(ref mut spike_file) = spike_file {
                write!(spike_file, "{} ", match action_potentials.get(entity) {
                    None => 0,
                    Some(_) => 1
                });
            }
        }

        if let Some(ref mut trace_file) = trace_file {
            writeln!(trace_file);
        }

        if let Some(ref mut spike_file) = spike_file {
            writeln!(spike_file);
        }
    }
}