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
$ cargo run --example single-neuron-trace > output.csv
   Compiling spiking-neural-net v0.1.0 (file:///path/to/spiking-neural-net)
    Finished dev [unoptimized + debuginfo] target(s) in 2.11 secs
     Running `target/debug/examples/single-neuron-trace`
0, -0.8389827395482743, -3.16059326818377, 4.380507094353632, 5
10, -0.8248061250730747, -3.0936129998394075, 4.380376416015061, 5
20, -0.8127511024678337, -3.0228727527871437, 4.380250920454535, 5
30, -0.8014456817973684, -2.9498986930201476, 4.380130076465763, 5
40, -0.7902885854182303, -2.875332912211439, 4.3800137222828734, 5
50, -0.7790063446801208, -2.7994385680578877, 4.379901860619658, 5
60, -0.7674664602317387, -2.7223137661322014, 4.379794570932519, 5
70, -0.7555971412972287, -2.643983632803613, 4.379691971608132, 5
80, -0.7433523092777098, -2.5644404689712217, 4.37959420375446, 5
...
```

This will produce a CSV file with five columns: the time (in milliseconds), followed by the potential of the four variables of the neuron:

* The membrane potential (mV)
* The voltage of the sodium-potassium ion channel (mV)
* The voltage of the slow ion channel (mV)
* The applied current (mA)

If you chart this data over time, you'll notice that the sodium-potassium channel exhibits a spiking pattern about twice a second.

### network-spike-log

This example creates a network of neurons and runs it. It logs a line any time one of the neurons emits an action potential (a 'spike'). In this case we're passing the `--real-time` flag so it runs the simulation at the same pace as the wall clock.

```
$ cargo run --example network-spike-log -- --size=10
   Compiling spiking-neural-net v0.1.0 (file:///path/to/spiking-neural-net)
    Finished dev [unoptimized + debuginfo] target(s) in 1.84 secs
     Running `target/debug/examples/network-spike-log --size=10`
t=660: neuron 8 is spiking
t=670: neuron 1 is spiking
t=680: neuron 9 is spiking
t=690: neuron 7 is spiking
t=700: neuron 8 is spiking
...
```

It records a spike whenever the voltage across the sodium-potassium ion channel (see the previous example) exceeds -3.5mV.
