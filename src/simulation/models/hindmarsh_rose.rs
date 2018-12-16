use specs::prelude::*;
use specs_derive::Component;
use serde_derive::Deserialize;
use log::info;

use crate::simulation::components::neuron::{
    ActionPotential,
    Neuron,
    Spiking,
};

#[derive(Component, Debug)]
pub struct HindmarshRoseModel {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub i: f64,
}


#[derive(Component, Debug, Deserialize)]
pub struct HindmarshRoseMorphology {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub beta: f64,
    pub r: f64,
    pub s: f64,
    pub x_r: f64,
    pub t_s: f64,
    pub epsp_amp: f64,
}

pub struct HindmarshRoseIntegrator;

impl <'a> System<'a> for HindmarshRoseIntegrator {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Neuron>,
        WriteStorage<'a, Spiking>,
        WriteStorage<'a, HindmarshRoseModel>,
        ReadStorage<'a, HindmarshRoseMorphology>
    );

    fn run(&mut self, (entities, updater, neurons, spikings, mut models, morphologies): Self::SystemData) {
        (&entities, &neurons, &mut models, &morphologies).par_join().for_each(|(entity, neuron, model, morphology)| {

            // Do two 0.5ms updates. This is supposedly for numerical stability, since we're doing
            // shitty-ass Euler integration.
            //
            // I took this idea from an implementation of the Izhikevich model, but haven't really
            // verified that it's necessary here.

            for _ in 0..2 {
                let sigma_x = -morphology.a * model.x.powi(3) + morphology.b * model.x.powi(2);
                let psi_x = morphology.c - morphology.d * model.x.powi(2);

                let dx_dt = model.y + sigma_x - model.z + model.i;
                let dy_dt = psi_x - morphology.beta * model.y;
                let dz_dt = morphology.r * (morphology.s * (model.x - morphology.x_r) - model.z);
                let di_dt = neuron.psp;

                model.x += 0.5 * (dx_dt / 1000.);
                model.y += 0.5 * (dy_dt / 1000.);
                model.z += 0.5 * (dz_dt / 1000.);
                model.i += 0.5 * (di_dt); // this is an impulse, so it doesn't get divided by 1000
            }

            if model.y > -3.5 {
                if spikings.get(entity).is_none() {
                    // we weren't already spiking, so mark a new action potential
                    updater.insert(entity, Spiking);
                    updater.insert(entity, ActionPotential);
                }
            } else if model.y <= -3.5 && spikings.get(entity).is_some() {
                updater.remove::<Spiking>(entity);
            }

            let spiking = spikings.get(entity);
            info!("{:?} {:?} {:?} {:?}", neuron, entity, spiking, model);
        });
    }
}
