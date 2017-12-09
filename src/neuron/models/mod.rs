
pub trait NeuronModel {
    fn new(id: usize, num_neurons: usize, dt: f64) -> Self;

    fn apply_epsps(&mut self, epsp_times: &Vec<f64>, time: f64);
    fn advance(&mut self, dt: f64);

    fn id(&self) -> usize;
    fn is_spiking(&self) -> bool;
}

mod hindmarsh_rose;
pub use self::hindmarsh_rose::HindmarshRoseNeuron;
