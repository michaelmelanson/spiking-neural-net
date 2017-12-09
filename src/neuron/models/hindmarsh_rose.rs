use std;
use rand;
use freude::{RungeKutta4, Stepper};
use tuple::T4;
use rayon::prelude::*;

use super::{NeuronModel, NeuronMorphology};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct HindmarshRoseParams {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    beta: f64,
    r: f64,
    s: f64,
    x_r: f64,
    t_s: f64,
    epsp_amp: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct HindmarshRoseSerializedState {
    x: f64,
    y: f64,
    z: f64,
    i: f64,
}

impl HindmarshRoseSerializedState {
    fn to_tuple(&self) -> T4<f64, f64, f64, f64> {
        T4(self.x, self.y, self.z, self.i)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HindmarshRoseMorphology {
    params: HindmarshRoseParams,
    state: HindmarshRoseSerializedState,
}

impl NeuronMorphology for HindmarshRoseMorphology {
    type Model = HindmarshRoseNeuron;
}

type HindmarshRoseStateTuple = T4<f64, f64, f64, f64>;
type HindmarshRoseFunction = Box<Fn(HindmarshRoseStateTuple) -> HindmarshRoseStateTuple>;

pub struct HindmarshRoseNeuron {
    id: usize,
    function: HindmarshRoseFunction,
    params: HindmarshRoseParams,
    pub state: HindmarshRoseStateTuple,
    weights: Vec<f64>,
    integrator: RungeKutta4<HindmarshRoseStateTuple>,
}

unsafe impl Send for HindmarshRoseNeuron {}
unsafe impl Sync for HindmarshRoseNeuron {}

impl NeuronModel<HindmarshRoseMorphology> for HindmarshRoseNeuron {
    fn new(id: usize, num_neurons: usize, dt: f64, morphology: &HindmarshRoseMorphology) -> Self {
        let params = morphology.params.clone();

        let function = hindmarsh_rose(params.clone());
        let state: HindmarshRoseStateTuple = morphology.state.to_tuple();

        let integrator = RungeKutta4::new(&state, dt);
        let mut weights = Vec::new();
        for _ in 0..num_neurons {
            weights.push((rand::random::<f64>() * 2.) - 1.);
        }

        HindmarshRoseNeuron {
            id: id,
            function: function,
            params: params,
            state: state,
            weights: weights,
            integrator: integrator,
        }
    }

    fn id(&self) -> usize {
        return self.id;
    }

    fn is_spiking(&self) -> bool {
        return self.state.1 > -3.5;
    }

    fn apply_epsps(&mut self, epsp_times: &Vec<f64>, time: f64) {
        assert_eq!(epsp_times.len(), self.weights.len());

        let weights = &self.weights;
        let epsp_amp = self.params.epsp_amp;
        let epsp_i: f64 = epsp_times.par_iter()
            .zip(weights.par_iter())
            .map(|(epsp_time, weight)| {
                let epsp_dt = time - epsp_time;
                let a = 1. / 3.;
                let epsp_amp = weight * epsp_amp;
                let decay = a * std::f64::consts::PI.sqrt() * -(epsp_dt / a).powi(2).exp();
                let dirac = epsp_amp / decay;

                -dirac / epsp_dt
            })
            .sum();

        // the '+ 5.' here adds a constant current to trigger the neuron to fire spontaneously
        self.state.3 = epsp_i + 5.;
    }

    fn advance(&mut self, dt: f64) {
        self.integrator.integrate_time(&mut self.function, &mut self.state, dt);
    }
}

fn hindmarsh_rose(params: HindmarshRoseParams) -> HindmarshRoseFunction {
    Box::new(move |T4(x, y, z, i): HindmarshRoseStateTuple| -> HindmarshRoseStateTuple {
        let sigma_x = -params.a * x.powi(3) + params.b * x.powi(2);
        let psi_x = params.c - params.d * x.powi(2);

        let dx_dt = y + sigma_x - z + i;
        let dy_dt = psi_x - params.beta * y;
        let dz_dt = params.r * (params.s * (x - params.x_r) - z);
        let di_dt = 0.;

        T4(dx_dt / params.t_s,
           dy_dt / params.t_s,
           dz_dt / params.t_s,
           di_dt / params.t_s)
    })
}
