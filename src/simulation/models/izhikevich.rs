use specs::prelude::*;
use specs_derive::Component;
use serde_derive::Deserialize;

use crate::simulation::components::neuron::{
    ActionPotential,
    Neuron
};

use rand::distributions::{
    Uniform,
    Distribution
};

#[derive(Component, Debug)]
pub struct IzhikevichModel {
    pub u: f64,
    pub v: f64
}


#[derive(Component, Clone, Debug, Deserialize)]
pub struct IzhikevichMorphology {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

pub struct IzhikevichIntegrator;

impl <'a> System<'a> for IzhikevichIntegrator {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteStorage<'a, Neuron>,
        WriteStorage<'a, IzhikevichModel>,
        ReadStorage<'a, IzhikevichMorphology>
    );

    fn run(&mut self, (entities, updater, mut neurons, mut models, morphologies): Self::SystemData) {
        let thalamic_input = Uniform::new(0., 5.);

        (&entities, &mut neurons, &mut models, &morphologies).par_join().for_each(|(entity, mut neuron, mut model, morphology)| {
            let i = neuron.psp + thalamic_input.sample(&mut rand::thread_rng());
            neuron.psp = 0.;

            model.v += 0.5 * ((0.04 * model.v * model.v) + (5. * model.v) + 140. - model.u + i);
            model.v += 0.5 * ((0.04 * model.v * model.v) + (5. * model.v) + 140. - model.u + i);
            model.u += morphology.a * ((morphology.b * model.v) - model.u);

            if model.v >= 30. {
                model.v = morphology.c;
                model.u += morphology.d;

                updater.insert(entity, ActionPotential);
            } else {
                updater.remove::<ActionPotential>(entity);
            }
        });
    }
}
