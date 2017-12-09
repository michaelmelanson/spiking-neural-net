# spiking-neural-net

This is a Rust library, plus some example tools, that simulates a spiking neural network. 

It currently supports a [Hindmarsh-Rose model](https://en.wikipedia.org/wiki/Hindmarsh%E2%80%93Rose_model) which can simulate most types of spiking behaviour seen in biological neurons.

Tests on my development machine show that it can simulate a network of about 1300 neurons in real-time when built in release mode.

## Status

- [x] Single neuron simulation
- [x] Network simulation with post-synaptic potentials
- [ ] Multiple neuron types (morphologies)
- [ ] Hebbian learning via spike timing dependent plasticity

## Usage

This is a Rust library, so you can include it in your projects to use it. There are also examples:

### single-neuron-trace

This example creates a single-neuron network and runs it. At each time step, in prints a line with the current state of the neuron.

```
$ cargo run --example single-neuron-trace -- --morphology data/morphologies/hindmarsh-rose/typical.yml > output.csv
   Compiling spiking-neural-net v0.1.0 (file:///path/to/spiking-neural-net)
    Finished dev [unoptimized + debuginfo] target(s) in 2.11 secs
     Running `target/debug/examples/single-neuron-trace --morphology data/morphologies/hindmarsh-rose/typical.yml`
```

This will produce a CSV file in `output.csv` with five columns: the time (in milliseconds), followed by the the four variables of the neuron model representing:

* The membrane potential (mV)
* The voltage of the sodium-potassium ion channel (mV)
* The voltage of the slow ion channel (mV)
* The applied current (mA)

If you chart this data over time, you'll notice that the sodium-potassium channel exhibits a spiking pattern about twice a second.

### network-spike-log

This example creates a network of neurons and runs it. It logs a line any time one of the neurons emits an action potential (a 'spike'). In this case we're passing the `--real-time` flag so it runs the simulation at the same pace as the wall clock.

```
$ cargo run --example network-spike-log -- --size=10 --morphology data/morphologies/hindmarsh-rose/typical.yml
   Compiling spiking-neural-net v0.1.0 (file:///path/to/spiking-neural-net)
    Finished dev [unoptimized + debuginfo] target(s) in 1.84 secs
     Running `target/debug/examples/network-spike-log --size=10 --morphology data/morphologies/hindmarsh-rose/typical.yml`
t=660: neuron 8 is spiking
t=670: neuron 1 is spiking
t=680: neuron 9 is spiking
t=690: neuron 7 is spiking
t=700: neuron 8 is spiking
...
```

It records a spike whenever the voltage across the sodium-potassium ion channel (see the previous example) exceeds -3.5mV.
