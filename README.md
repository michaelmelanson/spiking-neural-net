# spiking-neural-net [![Build Status](https://travis-ci.org/michaelmelanson/spiking-neural-net.svg?branch=master)](https://travis-ci.org/michaelmelanson/spiking-neural-net)
A spiking neural network simulation library.

## Usage

You will need a Rust development environment, which you can install by visiting https://rustup.rs/ and following the instructions.

Once you have Rust and Cargo installed, you can run a simulation with:

    make
  

You should see output like this:

![Sample output](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/sample%20output.png)

This will produce plot images called `neuron-trace.png` and `spikes.png` like the ones below.

Note that you can also build the debug version by omitting the `--release` flag, but it will run slowly. This is good if you want to use a debugger, but if you want to simulate any reasonably large networks in real-time you will need to use release mode.

## Sample output

It should create output images like this:

Filename      | Image
--------------|-----------
`neuron-trace.png` | ![spikes.png](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/neuron-trace.png) This shows the activity of a single neuron. Each of the vertical lines is an action potential, shown as a white dot on the plots below. There's a noise floor representing random thalamic input.
`spikes.png` | ![spikes.png](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/spikes.png) You'll need to download this image and zoom in to see the activity.
`spikes.png` (cropped) | ![spikes.png](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/spikes%20(cropped).png) This represents about 1s of time across 1100 neurons.

The `spikes.png` image shows some fascinating results: 
* In the cropped photo you can see vertical lines of dots. This shows that all the neurons in the network have synchronized and are pulsing together at about 12Hz. This is a similar rate to Alpha waves in mammalian brains.
* Also in the cropped photo can also see horizontal lines. This is the motor layer of each column; this layer currently is weakly connected and doesn't synchronize with the rest of the neurons.
* In the full `spikes.png` image there are three distinct phases:
  * **0s-4s:** The network quickly settles into a 5Hz rhythm, similar to Theta waves. I believe this is when the columns are organizing themselves.
  * **4s-10s:** The network becomes disorganized. I believe this is when the columns are learning to wire together and are trying to sort out their connections.
  * **10s+:** The network becomes synchronized at a much faster 12Hz rhythm.
  
## About the simulation

This library simulates networks of biologically-inspired neurons. Spiking neural models are implemented as ordinary differential equations integrated using Euler integration at 1 millisecond resolution.

The simuation is written in the Rust programming language and uses [Specs](https://github.com/slide-rs/specs), an Entity-Component-System framework with excellent parallelization and performance. This allows it to simulate simulate about 1800 neurons and 100k synapses, including an online learning algorithm, in real-time on a typical laptop. 

## Features

### Neural models

- [x] Izhikevich neurons
  * Model: https://www.izhikevich.org/publications/spikes.htm
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/models/izhikevich.rs
- [x] Hindmarsh-Rose neurons
  * Model: https://en.wikipedia.org/wiki/Hindmarsh%E2%80%93Rose_model
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/models/hindmarsh_rose.rs

### Learning models

- [x] Spike-timing dependent plasticity
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/learning/stdp.rs

### Network organization

- [x] Columnar organization
  * Networks are organized into 100-neuron columns composed of five layers (motor, sensory, afferent, efferent, internal)
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/mod.rs#L70-L160

## TODO
- [ ] Multiple morphologies. Currently all neurons are Izhikevich 'regular spiking' neurons.
- [ ] Configuration. Currently you need to change the source code to change the network design or neural model parameters.

